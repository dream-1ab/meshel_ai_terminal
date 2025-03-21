/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 03:53:19
 * @modify date 2025-03-22 03:53:19
 * @desc [description]
 */
use std::{io::Read, time::Duration};

use flutter_quickjs_bridge::{JsEngine, RustJsModule};
use quickjs_rusty::{q::{JS_Ext_NewSpecialValue, JS_TAG_EXCEPTION}, serde::from_js, utils::create_undefined, ExecutionError, OwnedJsValue, ToOwnedJsValue};
use serde_json::Value;

fn main() {
    let mut engine = JsEngine::new();
    init_native_modules(&mut engine);
    {
        engine.context.eval_module(&read_file("./assets/bootstrap.mjs"), true).unwrap();
        let result = engine.context.eval_module(&read_file("./assets/test.mjs"), true);
        match result {
            Ok(result) => {},
            Err(error) => {
                println!("{:?}", error);
                if let ExecutionError::Exception(error) = &error {
                    println!("{}", error.js_to_string().unwrap());
                }
            },
        }
    }
    println!("Done.");
}

fn read_file(file_name: &str) -> String {
    let file = std::fs::File::open(file_name).unwrap();
    let source_code = std::io::read_to_string(file).unwrap();
    source_code
}

fn init_native_modules(engine: &mut JsEngine) {
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
        module.register_function("setDartCallHandler", 0, |context, mut args: Vec<OwnedJsValue>, tag| {
            let first = args[0].clone();
            let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
            js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
            OwnedJsValue::new(context, create_undefined())
        });
        module.register_function("callDart", 0, |context, mut args: Vec<OwnedJsValue>, tag| {
            let first = args[0].clone();
            let js_function = first.try_into_function().expect("Cannot convert first parameter to function");
            js_function.call(vec![10.to_owned(context), ToOwnedJsValue::to_owned("Hello world", context)]).unwrap();
            OwnedJsValue::new(context, create_undefined())
        });
        module
    }).expect("Cannot register dart interop module");
}
