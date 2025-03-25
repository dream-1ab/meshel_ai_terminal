use std::os::raw::c_void;

use quickjs_rusty::{q::{JSRuntime, JS_GetException, JS_NewArrayBuffer}, utils::create_int, JsFunction, OwnedJsValue};

use crate::print_execution_error;

use super::ref_from_pointer::reference_from_boxed_pointer;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-24 15:09:39
 * @modify date 2025-03-24 15:09:39
 * @desc [description]
*/

#[repr(C)]
pub struct JavaScriptFunction {
    function: *const c_void,
}

impl JavaScriptFunction {
    pub fn new(function: JsFunction) -> *const JavaScriptFunction {
        let js_fn = Box::new(function);
        let wrapper = Box::new(JavaScriptFunction {
            function: Box::into_raw(js_fn) as *const _
        });
        Box::into_raw(wrapper)
    }

    pub fn get_internal_function_reference(&self) -> &JsFunction {
        reference_from_boxed_pointer(self.function as _)
    }
}

impl Drop for JavaScriptFunction {
    fn drop(&mut self) {
        let js_fn = unsafe {Box::from_raw(self.function as *mut JsFunction)};
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn javascript_function_free(javascript_function_ptr: *const JavaScriptFunction) {
    let wrapper = unsafe {Box::from_raw(javascript_function_ptr as *mut JavaScriptFunction)};
}

use std::os::raw::*;
#[unsafe(no_mangle)]
pub extern "C" fn javascript_function_call(javascript_function_ptr: *const JavaScriptFunction, action: i32, bytes_ptr: *const c_char, bytes_length: u32, tag: i32) -> i32 {
    let js_function = unsafe {
        (&*javascript_function_ptr).get_internal_function_reference()
    };
    let context = js_function.context();
    let buffer = unsafe {
        unsafe extern "C" fn free_buffer(rt: *mut JSRuntime, opaque: *mut ::std::os::raw::c_void ,ptr: *mut ::std::os::raw::c_void,) {
            //nothing here, because buffer is managed by dart vm and only valid during js function call.
            //using buffer from outside of javascript function is invalid and causes crash.
        }
        OwnedJsValue::new(context, JS_NewArrayBuffer(context, bytes_ptr as _, bytes_length as _, Some(free_buffer), std::ptr::null_mut(), 0))
    };
    let params = (
        OwnedJsValue::new(context, create_int(context, action)),
        buffer,
        OwnedJsValue::new(context, create_int(context, tag)),
    );
    let result = js_function.call(vec![params.0, params.1, params.2]);
    match result {
        Ok(result) => {
            if result.is_int() {
                result.to_int().unwrap()
            } else if result.is_exception() {
                let context = result.context();
                let exception = unsafe {
                    OwnedJsValue::new(context, JS_GetException(context)).try_into_object().expect("Cannot convert exception to object.")
                };
                let stack = exception.property("stack").unwrap().unwrap().js_to_string().unwrap();
                println!("JavaScript error: {}, stackTrace: \n{}", exception.js_to_string().unwrap(), stack);
                -1
            } else {
                -2
            }
        },
        Err(error) => {
            print_execution_error(error);
            -1
        },
    }
}
