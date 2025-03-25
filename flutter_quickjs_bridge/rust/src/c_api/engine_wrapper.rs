/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 15:19:18
 * @modify date 2025-03-22 15:19:18
 * @desc [description]
*/


use std::{ffi::{CStr, CString}, os::raw::*};

use flatbuffers::FlatBufferBuilder;
use quickjs_rusty::{q::{JS_GetArrayBuffer, JS_IsArrayBuffer, JS_Throw, JS_ThrowTypeError, JSValue}, utils::{create_int, create_undefined}, OwnedJsValue, RawJSValue,};

use crate::{c_api::javascript_function_wrapper::JavaScriptFunction, javascript_engine::RustJsModule, print_execution_error, DartFuncionCallCallback, DartJavascriptRegisterFunctionCallback, JavaScriptEngineDartWrapper};

use super::{javascript_engine_wrapper_generated::javascript_engine_dart_wrapper::{root_as_dart_module, DartModule, DartModuleBuilder}, raw_string::RawString, ref_from_pointer::reference_from_boxed_pointer};

#[unsafe(no_mangle)]
pub extern "C" fn javascript_engine_new(engine_id: u64) -> *mut c_void {
    let wrapper = Box::new(JavaScriptEngineDartWrapper::new(engine_id));
    Box::into_raw(wrapper) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn javascript_engine_free(engine_ptr: *mut c_void) {
    let ptr = engine_ptr as *mut JavaScriptEngineDartWrapper;
    let engine = unsafe {
        Box::from_raw(ptr)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn javascript_engine_eval(engine_ptr: *mut c_void, raw_string: *const RawString) {
    let engine: &JavaScriptEngineDartWrapper = reference_from_boxed_pointer(engine_ptr);
    let result = engine.engine.context.eval_module(unsafe {&*raw_string}.as_rust_str(), true);
    match result {
        Ok(_) => {},
        Err(error) => {
            print_execution_error(error);
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn javascript_engine_set_dart_callback_functions(engine_ptr: *mut c_void, dart_call_function_callback: u64, dart_register_javascript_function_callback: u64) {
    let engine: &mut JavaScriptEngineDartWrapper = reference_from_boxed_pointer(engine_ptr);

    //dart function signature.
    // Int32 Function(Int32 action, Pointer<Uint8> bytes_pointer, Uint32 length, Uint64 id, Int32 tag)

    let dart_callback_function: DartFuncionCallCallback = unsafe {
        std::mem::transmute(dart_call_function_callback as *const c_void)
    };
    let dart_register_javascript_function_callback: DartJavascriptRegisterFunctionCallback = unsafe {
        std::mem::transmute(dart_register_javascript_function_callback)
    };
    engine.dart_function_call_callback_function = Some(dart_callback_function);
    engine.dart_javascript_function_register_callback = Some(dart_register_javascript_function_callback);


    //Register javascript function as dart function.
    fn register_module(wrapper: &mut JavaScriptEngineDartWrapper) {
        let engine = &mut wrapper.engine;
        engine.register_native_module({
            let mut module = RustJsModule::new("dart/interop".into());
            // module.register_function("onMemoryBufferTransferFromDart", 0, |context, args: Vec<OwnedJsValue>, tag| {
            //     let first = args[0].clone();
            //     let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
            //     js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
            //     OwnedJsValue::new(context, create_undefined())
            // });
            // module.register_function("transferMemoryBufferToDart", 0, |context, args: Vec<OwnedJsValue>, tag| {
            //     let first = args[0].clone();
            //     let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
            //     js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
            //     OwnedJsValue::new(context, create_undefined())
            // });
            let register_javascript_function_handler = wrapper.dart_javascript_function_register_callback.clone();
            let engine_id = wrapper.engine_id;
            module.export_function("exportJavaScriptFunction", 0, move |context, args: Vec<OwnedJsValue>, tag| {
                let register_function_handler = register_javascript_function_handler;
                let engine_id = engine_id;
                if register_function_handler.is_none() {
                    return OwnedJsValue::new(context, unsafe {
                        JS_ThrowTypeError(context, CString::new("JavaScript function register callback on dart VM is not ready yet.").unwrap().as_ptr())
                    });
                }
                {//validate number of parameters
                    if args.len() != 2 {
                        return OwnedJsValue::new(context, unsafe {
                            JS_ThrowTypeError(context, CString::new("Function only accepts exactly 2 argument., first argument is string, second is function.").unwrap().as_ptr())
                        });
                    }
                }

                let params = {//retrieve parameters
                    (args[0].clone(), args[1].clone())
                };
                {//validate parameter data types.
                    let mut parameter_types_are_correct = true;
                    parameter_types_are_correct &= params.0.is_string();
                    parameter_types_are_correct &= params.1.is_function();
                    if !parameter_types_are_correct {
                        return OwnedJsValue::new(context, unsafe {JS_ThrowTypeError(context, CString::new("Parameters are incorrect, please ensure first argument is integer, second argument is array buffer, third argument is integer.").unwrap().as_ptr())});
                    }
                }
                let function_name = params.0.to_string().unwrap();
                let function_name = {
                    let function_name_bytes = function_name.as_bytes();
                    (function_name_bytes, function_name_bytes.len())
                };
                let result = register_function_handler.unwrap()(engine_id, function_name.0.as_ptr() as _, function_name.1 as u32, JavaScriptFunction::new(params.1.try_into_function().expect("Cannot convert JSValue to JSFunction.")));
                OwnedJsValue::new(context, create_int(context, result))
            });
            module
        }).expect("Cannot register dart interop module");
    }

    register_module(engine);
}

#[unsafe(no_mangle)]
pub extern "C" fn javascript_engine_register_dart_function(engine_ptr: *mut c_void, bytes: *const c_char, length: u64) {
    let engine: &mut JavaScriptEngineDartWrapper = reference_from_boxed_pointer(engine_ptr);
    let module = root_as_dart_module(unsafe {std::slice::from_raw_parts(bytes as *const u8, length as usize)}).expect("Cannot parse flatbuffer object.");
    // println!("Module is being to registered");
    engine.engine.register_native_module({
        let engine_id = engine.engine_id;
        let dart_callback_function = engine.dart_function_call_callback_function.clone();
        let mut native_module = RustJsModule::new(module.name().expect("Cannot get module name").into());
        module.functions().expect("Cannot get functions list from parameter during module registeration.").iter().for_each(|function| {
            let function_id = function.function_id();
            native_module.export_function(function.name().expect("Cannot get function name").into(), 0, move |context, args: Vec<OwnedJsValue>, tag| {
                let dart_callback_function = dart_callback_function;
                {//validate number of parameters
                    if dart_callback_function.is_none() {
                        return OwnedJsValue::new(context, unsafe {
                            JS_ThrowTypeError(context, CString::new("Dart callback function is not set yet.").unwrap().as_ptr())
                        });
                    }
                    if args.len() > 3 || args.len() == 0 {
                        return OwnedJsValue::new(context, unsafe {
                            JS_ThrowTypeError(context, CString::new("Function only accepts between (includes) 1 and 3 arguments").unwrap().as_ptr())
                        });
                    }
                }

                let params = {//retrieve parameters
                    let mut action = OwnedJsValue::new(context, create_int(context, 0));
                    if args.len() >= 2 {
                        action = args[0].clone();
                    }
                    let array_buffer = if args.len() == 1 {
                        args[0].clone()
                    } else {
                        args[1].clone()
                    };
                    let tag = if args.len() == 3 {
                        args[2].clone()
                    } else {
                        OwnedJsValue::new(context, create_int(context, 0))
                    };
                    (action, array_buffer, tag)
                };
                {//validate parameter data types.
                    let mut parameter_types_are_correct = true;
                    parameter_types_are_correct &= params.0.is_int();
                    parameter_types_are_correct &= unsafe {
                        let js_value = params.1.clone().extract();
                        let is_array_buffer = JS_IsArrayBuffer(js_value) == 1;
                        OwnedJsValue::new(context, js_value);
                        is_array_buffer
                    };
                    parameter_types_are_correct &= params.2.is_int();
                    if !parameter_types_are_correct {
                        return OwnedJsValue::new(context, unsafe {JS_ThrowTypeError(context, CString::new("Parameters are incorrect, please ensure first argument is integer, second argument is array buffer, third argument is integer.").unwrap().as_ptr())});
                    }
                }
                
                let (data_ptr, size) = unsafe {//get data pointer
                    let mut size = 0usize;
                    let js_value: JSValue = params.1.clone().extract();
                    let data_pointer = JS_GetArrayBuffer(context, &mut size, js_value);
                    OwnedJsValue::new(context, js_value); //this is safe because js_value is valied until this whole function returns and that object memory is managed by js engine.
                    if data_pointer == std::ptr::null_mut() {
                        println!("Cannot get underlying memory of ArrayBuffer.");
                    }
                    (data_pointer, size)
                };
                let function_id = function_id.clone();

                let result = dart_callback_function.unwrap()(engine_id, params.0.to_int().unwrap(), data_ptr, size as u32, function_id, params.2.to_int().unwrap());

                OwnedJsValue::new(context, create_int(context, result))
            });
        });
        native_module
    }).expect("Cannot register dart module as rust native module to js engine.");
}
