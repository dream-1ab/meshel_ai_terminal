/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 03:53:19
 * @modify date 2025-03-22 03:53:19
 * @desc [description]
 */
use std::{fs::File, io::Read, time::Duration};


use flutter_quickjs_bridge::{c_lib::engine_wrapper::{javascript_engine_free, javascript_engine_new, javascript_engine_register_dart_function}, javascript_engine::{self, JsEngine, RustJsModule}, JavaScriptEngineDartWrapper};
use quickjs_rusty::{q::{JS_Ext_NewSpecialValue, JS_TAG_EXCEPTION}, serde::from_js, utils::create_undefined, ExecutionError, OwnedJsValue, ToOwnedJsValue};
use serde_json::Value;

fn main() {
    for i in 0..100000 {
        println!("{}", i);
        let engine_ptr = javascript_engine_new();
        let buffer = {
            let mut buffer = vec![];
            File::open("/media/dream-lab/Development/Project/meshel_ai_terminal/flutter_quickjs_bridge/dart/dart_module.bin").unwrap().read_to_end(&mut buffer).unwrap();
            buffer
        };
        javascript_engine_register_dart_function(engine_ptr, buffer.as_ptr() as *const i8, buffer.len() as u64);
        javascript_engine_free(engine_ptr);
    }
    println!("End.");
    return;
    let mut engine = JavaScriptEngineDartWrapper::new();
    {
        
        let result = engine.engine.context.eval_module(&read_file("../quickjs_stdlib/bin/test.js"), true);
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
