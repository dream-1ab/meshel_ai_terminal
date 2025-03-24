/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 03:53:19
 * @modify date 2025-03-22 03:53:19
 * @desc [description]
 */
use std::{fs::File, io::Read, time::Duration};


use flutter_quickjs_bridge::{c_api::engine_wrapper::{javascript_engine_free, javascript_engine_new, javascript_engine_register_dart_function}, javascript_engine::{self, JsEngine, RustJsModule}, print_execution_error, JavaScriptEngineDartWrapper};
use quickjs_rusty::{q::{JS_Ext_NewSpecialValue, JS_TAG_EXCEPTION}, serde::from_js, utils::create_undefined, ExecutionError, OwnedJsValue, ToOwnedJsValue};
use serde_json::Value;

fn main() {
    let mut engine = JavaScriptEngineDartWrapper::new(0);
    {
        
        let result = engine.engine.context.eval_module(&read_file("../quickjs_stdlib/bin/test.js"), true);
        match result {
            Ok(result) => {},
            Err(error) => {
                print_execution_error(error);
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
