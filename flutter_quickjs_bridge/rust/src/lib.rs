use std::{ffi::{c_char, CString}, time::Duration};

use c_api::{javascript_function_wrapper::JavaScriptFunction, ref_from_pointer::reference_from_boxed_pointer};
use javascript_engine::{JsEngine, RustJsModule};
use quickjs_rusty::{q::{JSRuntime, JS_Ext_NewSpecialValue, JS_NewUint8Array, JS_ThrowTypeError, JS_TAG_EXCEPTION}, serde::from_js, utils::{create_int, create_undefined}, ExecutionError, OwnedJsValue, ToOwnedJsValue};
use serde_json::Value;


pub mod javascript_engine;
pub mod c_api;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-21 02:55:33
 * @modify date 2025-03-21 02:55:33
 * @desc [description]
*/

pub type DartFuncionCallCallback = extern "C" fn (u64 /*engine_id*/, i32 /*action*/, *mut u8/*bytes_pointer*/, u32/*bytes_length*/, u64/*callback_id*/, i32/*tag*/) -> i32;
pub type DartJavascriptRegisterFunctionCallback = extern "C" fn (u64/*engine_id*/, *const c_char/*function_name_ptr*/, u32/*function_name_length*/, *const JavaScriptFunction) -> i32;

pub struct JavaScriptEngineDartWrapper {
    pub engine: JsEngine,
    pub dart_function_call_callback_function: Option<DartFuncionCallCallback>,
    pub dart_javascript_function_register_callback: Option<DartJavascriptRegisterFunctionCallback>,
    pub engine_id: u64,
}

impl JavaScriptEngineDartWrapper {
    pub fn new(engine_id: u64) -> Self {
        let mut engine = JsEngine::new();
        JavaScriptEngineDartWrapper::init_dart_related_native_modules(&mut engine);
        JavaScriptEngineDartWrapper::init_bootstrap(&mut engine);
        let mut me = Self { engine: engine, dart_function_call_callback_function: None, dart_javascript_function_register_callback: None, engine_id };
        me
    }


    pub fn init_bootstrap(engine: &JsEngine) {
        let bootstrap_source = include_str!("../../quickjs_stdlib/bin/bootstrap.js");
        // println!("{}", bootstrap_source);
        engine.context.eval_module(bootstrap_source, true).unwrap();
    }

    pub fn init_dart_related_native_modules(engine: &mut JsEngine) {
        ///register console
        engine.register_native_module({
            let mut module = RustJsModule::new("core/console".into());
            let closure = |context, args: Vec<OwnedJsValue>, tag| {
                let args: Vec<Value> = args.iter().map(|item| from_js(context, &item).unwrap()).collect();
                let json = serde_json::to_string(&args).unwrap();
                let mut mode = "";
                match tag {
                    1 => {
                        mode = "print";
                    }
                    2 => {
                        mode = "log"
                    }
                    3 => {
                        mode = "info"
                    }
                    4 => {
                        mode = "warn"
                    }
                    5 => {
                        mode = "error"
                    }
                    _ => {
                        return OwnedJsValue::new(context, unsafe {JS_Ext_NewSpecialValue(JS_TAG_EXCEPTION, 1)});
                    }
                }
                println!("{} {}", mode, json);
                OwnedJsValue::new(context, create_undefined())
            };
            module.export_function("print", 1, closure);
            module.export_function("log", 2, closure);
            module.export_function("info", 3, closure);
            module.export_function("warn", 4, closure);
            module.export_function("error", 5, closure);
            module
        }).expect("Cannot register native modules");

        engine.register_native_module({
            let mut module = RustJsModule::new("core/threading".into());
            module.export_function("sleep", 0, |context, args: Vec<OwnedJsValue>, tag| {
                if args.len() != 1 {
                    return OwnedJsValue::new(context, unsafe {JS_Ext_NewSpecialValue(JS_TAG_EXCEPTION, 1)});
                }
                let value = args.first().unwrap();
                if !value.is_int() {
                    return OwnedJsValue::new(context, unsafe {JS_Ext_NewSpecialValue(JS_TAG_EXCEPTION, 2)});
                }
                std::thread::sleep(Duration::from_millis(value.to_int().unwrap() as u64));
                OwnedJsValue::new(context, create_undefined())
            });
            module
        }).expect("Cannot register threading module");
        engine.register_native_module({
            use quickjs_rusty::q::*;
            let mut module = RustJsModule::new("core/rust".into());
            module.export_function("utf8_encode", 0, |context, args: Vec<OwnedJsValue>, tag|{
                if args.len() != 1 && !args[0].is_string() {
                    return OwnedJsValue::new(context, unsafe {
                        JS_ThrowTypeError(context, CString::new("argument is should be string and only accepts exactly one argument.").unwrap().as_ptr())
                    });
                }
                let text = Box::into_raw(Box::new(args[0].to_string().expect("Cannot convert argument to String"))) as _;
                let text_ref: &mut String = reference_from_boxed_pointer(text);
                let bytes = unsafe {text_ref.as_bytes_mut()};
                unsafe extern "C" fn free_buffer(rt: *mut JSRuntime, opaque: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void,) {
                    let underlying_text  = Box::from_raw(opaque as *mut String);
                }
                let uint8_array = unsafe {
                    let uint8_array = JS_NewUint8Array(context, bytes.as_mut_ptr(), bytes.len(), Some(free_buffer), text, 0);
                    OwnedJsValue::new(context, uint8_array)
                };
                uint8_array
            });
            module.export_function("utf8_decode", 0, |context, args: Vec<OwnedJsValue>, tag| {
                if args.len() != 1 {
                    let is_uint8_array = unsafe {
                        let first_arg = args[0].clone().extract();
                        let is_true = JS_IsArrayBuffer(first_arg) == 1;
                        OwnedJsValue::new(context, first_arg);
                        is_true
                    };
                    if !is_uint8_array {
                        return OwnedJsValue::new(context, unsafe {
                            JS_ThrowTypeError(context, CString::new("argument is should be string and only accepts exactly one argument.").unwrap().as_ptr())
                        });
                    }
                }
                let uint8_array = unsafe {
                    let mut size = 0;
                    let first_arg = args[0].clone().extract();
                    let pointer = JS_GetArrayBuffer(context, &mut size, first_arg);
                    OwnedJsValue::new(context, first_arg);
                    if pointer == std::ptr::null_mut() {
                        return OwnedJsValue::new(context, unsafe {
                            JS_ThrowTypeError(context, CString::new("Cannot decode utf8 because parameter is not Uint8Array or SharedArrayBuffer.").unwrap().as_ptr())
                        });
                    }
                    (pointer, size)
                };
                let byte_array = unsafe {
                    std::slice::from_raw_parts(uint8_array.0, uint8_array.1)
                };
                let result = match std::str::from_utf8(byte_array) {
                    Ok(text) => {
                        text.to_string().to_owned(context)
                    },
                    Err(error) => unsafe {
                        OwnedJsValue::new(context, JS_ThrowTypeError(context, CString::new(format!("error while decoding utf8 string: {:?}", error)).unwrap().as_ptr()))
                    },
                };
                result
            });
            module
        }).expect("Cannot register encoding/decoding modules");
    }
}

pub fn print_execution_error(error: ExecutionError) {
    println!("{:?}", error);
    match error {
        quickjs_rusty::ExecutionError::InputWithZeroBytes => {},
        quickjs_rusty::ExecutionError::Conversion(value_error) => {

        },
        quickjs_rusty::ExecutionError::Internal(_) => {},
        quickjs_rusty::ExecutionError::Exception(owned_js_value) => {
            let error = owned_js_value.js_to_string().unwrap();
            println!("{}", error);
        },
        quickjs_rusty::ExecutionError::OutOfMemory => {
            println!("Out of memory")                    
        },
        _ => {},
    }
}
