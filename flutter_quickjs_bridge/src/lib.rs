use std::{any::TypeId, ffi::{CStr, CString}, mem::transmute, rc::Rc, slice::from_raw_parts, time::Duration};

use quickjs_rusty::{q::{JSCFunctionData, JSModuleDef, JSValue, JS_AddModuleExport, JS_Ext_GetPtr, JS_Ext_NewCFunction, JS_Ext_NewPointer, JS_Ext_NewSpecialValue, JS_FreeValue, JS_GetOpaque, JS_GetRuntime, JS_GetRuntimeOpaque, JS_NewCFunction2, JS_NewCFunctionData, JS_NewCModule, JS_SetModuleExport, JS_SetOpaque, JS_SetRuntimeOpaque, JS_TAG_NULL, JS_TAG_UNDEFINED}, serde::from_js, utils::{create_empty_object, create_function, create_undefined}, Context, JSContext, JsTag, OwnedJsValue};
use serde_json::Value;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-21 02:55:33
 * @modify date 2025-03-21 02:55:33
 * @desc [description]
*/

type RUST_CONSOLE_HANDLER = dyn Fn(i32, Vec<Value>) -> u32;

pub struct JsEngine {
    pub context: Context,
    console: Box<dyn JsConsole>,
    console_handler: Option<Box<RUST_CONSOLE_HANDLER>>,
    modules: Vec<Box<RustJsModule>>,
}

pub trait JsConsole {
    fn log(&self, value: Vec<Value>);
    fn warn(&self, value: Vec<Value>);
    fn info(&self, value: Vec<Value>);
    fn error(&self, value: Vec<Value>);
}

impl JsEngine {
    pub fn new(console: impl JsConsole + 'static) -> Self {
        let context = Context::new(None).unwrap();
        let mut me = Self { context, console: Box::new(console), console_handler: None, modules: vec![] };
        me.initialize();
        me
    }

    fn initialize(&mut self) {
        self.initialize_console();
        self.initialize_modules();
    }

    fn initialize_console(&mut self) {
        let global = self.context.global().expect("Cannot get global object from js context");
        let context = unsafe {
            self.context.context_raw()
        };
        let console_object = OwnedJsValue::new(context, create_empty_object(context).expect("Cannot create console object")).try_into_object().unwrap();
        {
            type QJS_CONSOLE_HANDLER = unsafe extern "C" fn (context: *mut JSContext, this_val: JSValue, argc: ::std::os::raw::c_int, argv: *mut JSValue, magic: ::std::os::raw::c_int, func_data: *mut JSValue,) -> JSValue;

            fn make_handler<F>(f: F,) -> (QJS_CONSOLE_HANDLER, Box<dyn Fn(i32, Vec<Value>) -> u32>) where F: Fn(i32, Vec<Value>) -> u32 + 'static {
                unsafe extern "C" fn console_handler<F>(context: *mut JSContext, this_val: JSValue, argc: ::std::os::raw::c_int, argv: *mut JSValue, magic: ::std::os::raw::c_int, func_data: *mut JSValue,) -> JSValue where F: Fn(i32, Vec<Value>) -> u32 {
                    let args: Vec<Value> = (0..argc).into_iter().map(|i| unsafe {
                        let value = OwnedJsValue::own(context, &*argv.add(i as usize));
                        from_js(context, &value).unwrap()
                    }).collect();
                    let console = unsafe {
                        let ptr = JS_Ext_GetPtr(*func_data);
                        &*(ptr as *const F)
                    };
                    let result = console(magic as i32, args);
                    if result != 0 {
                        return unsafe {
                            JS_Ext_NewSpecialValue(quickjs_rusty::q::JS_TAG_EXCEPTION, result)
                        };
                    }
                    create_undefined()
                }
                (console_handler::<F>, Box::new(f))
            }

            let (handler, closure) = {
                let console_impl_ptr = self.console.as_ref() as *const dyn JsConsole;
                
                make_handler(move |action, args| {
                    let console_impl_ptr = console_impl_ptr;
                    let console_instance = unsafe {&*console_impl_ptr};
                    match action {
                        1 => {
                            console_instance.log(args);
                        }
                        2 => {
                            console_instance.warn(args);
                        }
                        3 => {
                            console_instance.info(args);
                        }
                        4 => {
                            console_instance.error(args);
                        }
                        _ => {
                            return 1;
                        }
                    }
                    0
                })
            };
            let mut console_handler_pointer = unsafe {
                let ptr = closure.as_ref() as *const dyn Fn(i32, Vec<Value>) -> u32 as *mut std::os::raw::c_void;
                println!("Closure pointer: {}", ptr as usize);
                JS_Ext_NewPointer(quickjs_rusty::q::JS_TAG_NULL, ptr)
            };
            self.console_handler = Some(closure);
            console_object.set_property("log", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(handler), 7, 1, 1, &mut console_handler_pointer)})).expect("Cannot set log function to console.");
            console_object.set_property("warn", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(handler), 7, 2, 1, &mut console_handler_pointer)})).expect("Cannot set warn function to console.");
            console_object.set_property("info", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(handler), 7, 3, 1, &mut console_handler_pointer)})).expect("Cannot set info function to console.");
            console_object.set_property("error", OwnedJsValue::new(context, unsafe {JS_NewCFunctionData(context, Some(handler), 7, 4, 1, &mut console_handler_pointer)})).expect("Cannot set error function to console.");
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

    pub fn register_native_module(&mut self, module: RustJsModule) -> Result<(), String> {
        let module = Box::new(module);
        let context = unsafe {self.context.context_raw()};
        let runtime = unsafe {JS_GetRuntime(context)};
        let module_name = CString::new(module.name.as_str()).unwrap();
        unsafe {
            extern "C" fn module_init(ctx: *mut JSContext, m: *mut JSModuleDef) -> ::std::os::raw::c_int {
                let mut result = 0;
                let rust_js_module = unsafe {&*(JS_GetRuntimeOpaque(JS_GetRuntime(ctx)) as *const RustJsModule)};
                for function in &rust_js_module.exported_functions {
                    let name = CString::new(function.function_name.as_str()).unwrap();
                    let mut closure_value = unsafe {
                        JS_Ext_NewPointer(JS_TAG_NULL, function.closure.as_ref() as *const dyn Fn(*mut JSContext, OwnedJsValue, Vec<OwnedJsValue>, i32) -> OwnedJsValue as *mut std::os::raw::c_void)
                    };
                    result = unsafe {JS_SetModuleExport(ctx, m, name.as_ptr(), JS_NewCFunctionData(ctx, function.js_c_function.clone(), function.rust_function.argument_count() as i32, function.tag, 1, &mut closure_value))};
                    unsafe {JS_FreeValue(ctx, closure_value)};
                }
                result
            }
            JS_SetRuntimeOpaque(runtime, module.as_ref() as *const RustJsModule as *mut std::os::raw::c_void);
            let def = JS_NewCModule(context, module_name.as_ptr(), Some(module_init));
            if def == std::ptr::null_mut() {
                return Err(format!("Cannot create CModule with JS_NewCModule with result: {}", def as usize));
            }
            for f in &module.exported_functions {
                let function_name = CString::new(f.function_name.as_str()).unwrap();
                let result = JS_AddModuleExport(context, def, function_name.as_ptr());
                if result != 0 {
                    return Err(format!("Cannot add module export with function: JS_AddModuleExport, result: {}", result));
                }
            }
        }
        self.modules.push(module);
        Ok(())
    }
}

pub struct RustJsModule {
    pub name: String,
    pub exported_functions: Vec<ExportedFunction>
}

pub struct ExportedFunction {
    pub function_name: String,
    pub rust_function: Box<dyn RustJsFunction>,
    pub closure: Box<dyn Fn(*mut JSContext, OwnedJsValue, Vec<OwnedJsValue>, i32) -> OwnedJsValue>,
    pub js_c_function: quickjs_rusty::q::JSCFunctionData,
    pub tag: i32
}

impl RustJsModule {
    pub fn new(module_name: String) -> Self {
        RustJsModule { name: module_name, exported_functions: vec![] }
    }

    pub fn register_function<RF>(&mut self, name: &str, tag: i32, f: RF) where RF: RustJsFunction + 'static {
        let rust_function = Box::new(f);
        fn make_rust_closure<F>(f: F) -> (JSCFunctionData, Box<F>) where F: Fn(*mut JSContext, OwnedJsValue, Vec<OwnedJsValue>, i32) -> OwnedJsValue {
            unsafe extern "C" fn js_rust_callback<F>(context: *mut JSContext, this_val: JSValue, argc: ::std::os::raw::c_int, argv: *mut JSValue, magic: ::std::os::raw::c_int, func_data: *mut JSValue,) -> JSValue where F: Fn(*mut JSContext, OwnedJsValue, Vec<OwnedJsValue>, i32) -> OwnedJsValue {
                let closure = unsafe {
                    let ptr = JS_Ext_GetPtr(*func_data.add(0));
                    &*(ptr as *const F)
                };
                let args: Vec<_> = (0..argc).into_iter().map(|item| OwnedJsValue::own(context, unsafe {&*argv.add(item as usize)})).collect();
                let result = closure(context, OwnedJsValue::own(context, &this_val), args, magic as i32);
                unsafe {result.extract()}
            }
            (Some(js_rust_callback::<F>), Box::new(f))
        }

        let rust_trait_object_pointer = rust_function.as_ref() as *const RF;
        let (js_c_function, rust_closure) = make_rust_closure(move |context, this_value, args, magic|{
            let traint_object_pointer = rust_trait_object_pointer;
            let instance = unsafe {&*traint_object_pointer};
            let result = instance.call(context, this_value, args, magic as i32);
            result
        });
        self.exported_functions.push(ExportedFunction { rust_function, closure: rust_closure, js_c_function: js_c_function, function_name: name.into(), tag });
    }
}

pub trait RustJsFunction {
    fn argument_count(&self) -> u32;
    fn call(&self, context: *mut JSContext, this_val: OwnedJsValue, args: Vec<OwnedJsValue>, tag: i32) -> OwnedJsValue;
}

impl<T> RustJsFunction for T where T: Fn(*mut JSContext, Vec<OwnedJsValue>, i32) -> OwnedJsValue {
    fn argument_count(&self) -> u32 {
        7
    }

    fn call(&self, context: *mut JSContext, this_val: OwnedJsValue, args: Vec<OwnedJsValue>, tag: i32) -> OwnedJsValue {
        self(context, args, tag)
    }
}
