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


// type FINAL_FN = fn(ptr: *mut ());

// fn make_fn() -> FINAL_FN {
//     fn final_fn(ptr: *mut (), ) {
//         let closure = ptr as *mut Box<dyn Fn()>;

//         unsafe {
//             (*closure)();
//         }
//     }

//     final_fn
// }

// fn consume_closure(){
//     let handle = make_fn();
//     let closure: *mut Box<dyn Fn()> = Box::into_raw(Box::new(Box::new(|| {
//         println!("Inside of closure");
//     })));
//     handle(closure as *mut ())
// }


// fn main() {
//     consume_closure();
//     // main2();
// }


// type Callback = extern "C" fn(*mut ());

// // Implemented somewhere in a library
// fn some_c_function(callback: Callback, argument: *mut ()) {
//     callback(argument);
// }

// extern fn my_callback(argument: *mut ()) {
//     unsafe {
//         let closure = argument as *mut Box<dyn Fn()>;
        
//         // Or, to destroy the box:
//         // let closure = Box::from_raw(argument as *mut Box<dyn Fn()>);
        
//         (*closure)();
//     }
// }

// fn main2() {
//     // let closure: Box<dyn Fn()> = Box::new(|| println!("hello"));
//     // let wrapped: Box<Box<dyn Fn()>> = Box::new(closure);
//     let wrapped: Box<Box<dyn Fn()>> = Box::new(Box::new(|| println!("Hello")));
//     let closure_raw = Box::into_raw(wrapped);

//     unsafe {
//         some_c_function(my_callback, closure_raw as *mut ());
//     }
// }
