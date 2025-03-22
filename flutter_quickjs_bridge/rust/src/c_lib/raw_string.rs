/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 15:21:30
 * @modify date 2025-03-22 15:21:30
 * @desc [description]
*/

use std::os::raw::*;

#[repr(C)]
pub struct RawString {
    pub ptr: *const c_char,
    pub length: u32,
}

impl RawString {
    pub fn as_rust_str(&self) -> &str {
        let bytes = unsafe {
            std::slice::from_raw_parts(self.ptr as *mut u8, self.length as usize)
        };
        std::str::from_utf8(bytes).expect("Cannot decode utf8 string.")
    }
}

pub struct DartFunction {
    function_name: RawString,
    pointer: u64
}

impl DartFunction {
    
}