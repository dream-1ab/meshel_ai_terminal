use std::io::Read;

use flutter_quickjs_bridge::{JsConsole, JsEngine};
use quickjs_rusty::ExecutionError;

fn main() {
    let engine = JsEngine::new(RustTerminalConsole);
    {
        let file = std::fs::File::open("./assets/test.mjs").unwrap();
        let source_code = std::io::read_to_string(file).unwrap();
        let result = engine.context.eval_module(&source_code, true);
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

struct RustTerminalConsole;

impl JsConsole for RustTerminalConsole {
    fn log(&self, value: Vec<serde_json::Value>) {
        let text = serde_json::to_string(&value).unwrap();
        println!("log: {}", text);
    }

    fn warn(&self, value: Vec<serde_json::Value>) {
        println!("warn: {:?}", value);
    }

    fn info(&self, value: Vec<serde_json::Value>) {
        println!("info: {:?}", value);
    }

    fn error(&self, value: Vec<serde_json::Value>) {
        println!("error: {:?}", value);
    }
}
