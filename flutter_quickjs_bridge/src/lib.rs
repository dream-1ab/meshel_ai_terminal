use std::{any::TypeId, ffi::{CStr, CString}, mem::transmute, rc::Rc, slice::from_raw_parts, time::Duration};

use quickjs_rusty::{q::*, serde::from_js, utils::{create_empty_object, create_function, create_undefined}, Context, JSContext, JsTag, OwnedJsValue};
use serde_json::Value;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-21 02:55:33
 * @modify date 2025-03-21 02:55:33
 * @desc [description]
*/

pub struct JsEngine {
    pub context: Context,
    modules: Vec<Box<RustJsModule>>,
}

impl JsEngine {
    pub fn new() -> Self {
        let context = Context::new(None).unwrap();
        let mut me = Self { context, modules: vec![] };
        me
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
                        JS_Ext_NewPointer(JS_TAG_FUNCTION_BYTECODE, function.closure.as_ref() as *const dyn Fn(*mut JSContext, OwnedJsValue, Vec<OwnedJsValue>, i32) -> OwnedJsValue as *mut std::os::raw::c_void)
                    };
                    result = unsafe {JS_SetModuleExport(ctx, m, name.as_ptr(), JS_NewCFunctionData(ctx, function.js_c_function.clone(), function.rust_function.argument_count() as i32, function.tag, 1, &mut closure_value))};
                    if result != 0 {
                        eprintln!("JS_SetModuleExport returns {} on module function {}", result, function.function_name)
                    }
                    unsafe {JS_FreeValue(ctx, closure_value)};
                }
                unsafe {
                    JS_SetRuntimeOpaque(JS_GetRuntime(ctx), std::ptr::null_mut());
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
        //force to load modules because module init is lazy, this prevents second registeration of module will override first module pointer in runtime.
        self.context.eval_module(format!("import {{}} from \"{}\"", module.name).as_str(), true).expect("Cannot import modules.");
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
