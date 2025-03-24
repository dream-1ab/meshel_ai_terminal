use std::{ffi::{c_char, CString}, time::Duration};

use c_api::javascript_function_wrapper::JavaScriptFunction;
use javascript_engine::{JsEngine, RustJsModule};
use quickjs_rusty::{q::{JS_Ext_NewSpecialValue, JS_ThrowTypeError, JS_TAG_EXCEPTION}, serde::from_js, utils::{create_int, create_undefined}, ExecutionError, OwnedJsValue, ToOwnedJsValue};
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
            module.register_function("print", 1, closure);
            module.register_function("log", 2, closure);
            module.register_function("info", 3, closure);
            module.register_function("warn", 4, closure);
            module.register_function("error", 5, closure);
            module
        }).expect("Cannot register native modules");

        engine.register_native_module({
            let mut module = RustJsModule::new("core/threading".into());
            module.register_function("sleep", 0, |context, args: Vec<OwnedJsValue>, tag| {
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