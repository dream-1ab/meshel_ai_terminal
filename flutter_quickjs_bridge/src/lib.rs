use std::{ffi::{CStr, CString}, mem::transmute, rc::Rc, slice::from_raw_parts, time::Duration};

use quickjs_rusty::{q::{JSModuleDef, JSValue, JS_AddModuleExport, JS_Ext_GetPtr, JS_Ext_NewCFunction, JS_Ext_NewPointer, JS_Ext_NewSpecialValue, JS_GetOpaque, JS_NewCFunction2, JS_NewCFunctionData, JS_NewCModule, JS_SetModuleExport, JS_SetOpaque}, serde::from_js, utils::{create_empty_object, create_function, create_undefined}, Context, JSContext, JsTag, OwnedJsValue};
use serde_json::Value;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-21 02:55:33
 * @modify date 2025-03-21 02:55:33
 * @desc [description]
*/


pub struct JsEngine<CONSOLE> {
    pub context: Context,
    console: Box<CONSOLE>
}

pub trait JsConsole {
    fn log(&self, value: Vec<Value>);
    fn warn(&self, value: Vec<Value>);
    fn info(&self, value: Vec<Value>);
    fn error(&self, value: Vec<Value>);
}

impl<CONSOLE> JsEngine<CONSOLE> where CONSOLE: JsConsole {
    pub fn new(console: CONSOLE) -> Self {
        let context = Context::new(None).unwrap();
        let me = Self { context, console: Box::new(console) };
        me.initialize();
        me
    }

    fn initialize(&self) {
        self.initialize_console();
        self.initialize_modules();
    }

    fn initialize_console(&self) {
        let global = self.context.global().expect("Cannot get global object from js context");
        let context = unsafe {
            self.context.context_raw()
        };
        let console_object = OwnedJsValue::new(context, create_empty_object(context).expect("Cannot create console object")).try_into_object().unwrap();
        {
            unsafe extern "C" fn console_handler<T>(context: *mut JSContext, this_val: JSValue, argc: ::std::os::raw::c_int, argv: *mut JSValue, magic: ::std::os::raw::c_int, func_data: *mut JSValue,) -> JSValue where T: JsConsole {
                let args: Vec<Value> = (0..argc).into_iter().map(|i| unsafe {
                    let value = OwnedJsValue::own(context, &*argv.add(i as usize));
                    from_js(context, &value).unwrap()
                }).collect();
                let console = unsafe {
                    let ptr = JS_Ext_GetPtr(*func_data);
                    &*(ptr as *const T)
                };
                match magic {
                    1 => { //log
                        console.log(args);
                    }
                    2 => { //warn
                        console.warn(args);
                    }
                    3 => { //info
                        console.info(args);
                    }
                    4 => { //error
                        console.error(args);
                    }
                    _ => {
                        panic!("Unknown magic number for console functions.");
                    }
                }
                create_undefined()
            }
            let mut console_object_pointer = unsafe {
                // let ptr = self.console.as_ref() as *const dyn JsConsole as *mut std::os::raw::c_void;
                let ptr = self.console.as_ref() as *const dyn JsConsole as *mut std::os::raw::c_void;
                JS_Ext_NewPointer(quickjs_rusty::q::JS_TAG_NULL, ptr)
            };
            console_object.set_property("log", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(console_handler::<CONSOLE>), 7, 1, 1, &mut console_object_pointer)})).expect("Cannot set log function to console.");
            console_object.set_property("warn", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(console_handler::<CONSOLE>), 7, 2, 1, &mut console_object_pointer)})).expect("Cannot set warn function to console.");
            console_object.set_property("info", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(console_handler::<CONSOLE>), 7, 3, 1, &mut console_object_pointer)})).expect("Cannot set info function to console.");
            console_object.set_property("error", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(console_handler::<CONSOLE>), 7, 4, 1, &mut console_object_pointer)})).expect("Cannot set error function to console.");
        }
        global.set_property("console", console_object.into_value()).expect("Cannot set console object.");
    }

    fn initialize_modules(&self) {
        let context = unsafe {self.context.context_raw()};
        unsafe {//threading related
            let module_name = CString::new("core/threading").unwrap();
            let module_def = JS_NewCModule(context, module_name.as_ptr(), Some({
                unsafe extern "C" fn module_init(context: *mut JSContext, m: *mut JSModuleDef) -> std::os::raw::c_int {
                    let function_name = CString::new("sleep").unwrap();
                    // println!("m is null = {}", m == std::ptr::null_mut());
                    unsafe {
                        let result = JS_SetModuleExport(context, m, function_name.as_ptr(), JS_Ext_NewCFunction(context, Some({
                            unsafe extern "C" fn sleep(context: *mut JSContext, this_val: JSValue, argc: ::std::os::raw::c_int, argv: *mut JSValue) -> JSValue {
                                // println!("Hit to sleep.");
                                if argc != 1 {
                                    return unsafe {JS_Ext_NewSpecialValue(quickjs_rusty::q::JS_TAG_EXCEPTION, 1)};
                                }
                                let owned_value = OwnedJsValue::own(context, &*argv.add(0));
                                if !owned_value.is_int() {
                                    return unsafe {JS_Ext_NewSpecialValue(quickjs_rusty::q::JS_TAG_EXCEPTION, 2)};
                                }
                                std::thread::sleep(Duration::from_millis(owned_value.to_int().unwrap() as u64));
                                return create_undefined();
                            }
                            sleep
                        }), function_name.as_ptr(), 1));
                        if result != 0 {
                            println!("Cannot export function sleep with error code: {}", result);
                        }
                    };
                    return 0;
                }
                module_init
            }));
            if module_def == std::ptr::null_mut() {
                panic!("Rust Js module creation failed")
            }
            if JS_AddModuleExport(context, module_def, CString::new("sleep").unwrap().as_ptr()) != 0 {
                panic!("JS_AddModuleExport failed")
            }
        }
    }
}
