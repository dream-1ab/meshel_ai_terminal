/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 15:19:18
 * @modify date 2025-03-22 15:19:18
 * @desc [description]
*/


use std::os::raw::*;

use crate::JavaScriptEngineDartWrapper;

use super::{raw_string::RawString, ref_from_pointer::reference_from_boxed_pointer};

#[unsafe(no_mangle)]
extern "C" fn javascript_engine_new() -> *mut c_void {
    let wrapper = Box::new(JavaScriptEngineDartWrapper::new());
    Box::into_raw(wrapper) as *mut c_void
}

#[unsafe(no_mangle)]
extern "C" fn javascript_engine_free(engine_ptr: *mut c_void) {
    let ptr = engine_ptr as *mut JavaScriptEngineDartWrapper;
    let engine = unsafe {
        Box::from_raw(ptr)
    };
}

#[unsafe(no_mangle)]
extern "C" fn javascript_engine_eval(engine_ptr: *mut c_void, source_code_ptr: *const c_char, source_code_length: c_uint) {
    let raw_string = RawString {length: source_code_length, ptr: source_code_ptr};
    let engine: &JavaScriptEngineDartWrapper = reference_from_boxed_pointer(engine_ptr);
    
    engine.engine.context.eval_module(raw_string.as_rust_str(), true).expect("Cannot execute javascript code");
}

#[unsafe(no_mangle)]
extern "C" fn javascript_engine_get_javascript_function(engine_ptr: *mut c_void, function_name_ptr: *const c_char, function_name_length: c_uint) {
    let engine: &JavaScriptEngineDartWrapper = reference_from_boxed_pointer(engine_ptr);
}
