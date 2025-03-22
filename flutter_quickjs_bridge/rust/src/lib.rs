use std::{ffi::c_char, time::Duration};

use javascript_engine::{JsEngine, RustJsModule};
use quickjs_rusty::{q::{JS_Ext_NewSpecialValue, JS_TAG_EXCEPTION}, serde::from_js, utils::create_undefined, OwnedJsValue, ToOwnedJsValue};
use serde_json::Value;


pub mod javascript_engine;
pub mod c_lib;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-21 02:55:33
 * @modify date 2025-03-21 02:55:33
 * @desc [description]
*/

// Int32 Function(Uint32 action, Pointer<Uint8> bytes_pointer, Uint32 length, Uint32 id, Uint32 tag)
pub type DartCallbackFunction = extern "C" fn (i32, *mut u8, u32, u64, i32) -> i32;
pub struct JavaScriptEngineDartWrapper {
    pub engine: JsEngine,
    pub dart_callback_function: Option<DartCallbackFunction>,
}

impl JavaScriptEngineDartWrapper {
    pub fn new() -> Self {
        let mut engine = JsEngine::new();
        JavaScriptEngineDartWrapper::init_dart_related_native_modules(&mut engine);
        JavaScriptEngineDartWrapper::init_bootstrap(&engine);
        Self { engine: engine, dart_callback_function: None }
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
            module.register_function("onDartFunctionTransferFromDart", 0, |context, args: Vec<OwnedJsValue>, tag| {
                let first = args[0].clone();
                let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
                js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
                OwnedJsValue::new(context, create_undefined())
            });
            module.register_function("transferJavascriptFunctionToDart", 0, |context, args: Vec<OwnedJsValue>, tag| {
                let first = args[0].clone();
                let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
                js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
                OwnedJsValue::new(context, create_undefined())
            });
            module.register_function("callDartFunction", 0, |context, args: Vec<OwnedJsValue>, tag| {
                let first = args[0].clone();
                let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
                js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
                OwnedJsValue::new(context, create_undefined())
            });
            module
        }).expect("Cannot register dart interop module");
    }

}

