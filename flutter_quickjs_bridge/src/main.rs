use std::io::Read;

use flutter_quickjs_bridge::{JsConsole, JsEngine};

fn main() {
    let engine = JsEngine::new(RustTerminalConsole);
    {
        let file = std::fs::File::open("./assets/test.mjs").unwrap();
        let source_code = std::io::read_to_string(file).unwrap();
        engine.context.eval_module(&source_code, true).unwrap();
    }
    println!("Done.");
}

struct RustTerminalConsole;

impl JsConsole for RustTerminalConsole {
    fn log(&self, value: Vec<serde_json::Value>) {
        println!("log: {:?}", value);
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
