/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 15:38:48
 * @modify date 2025-03-22 15:38:48
 * @desc [description]
*/


///This function doesn't take underlaying pointer ownership (doesn't change pointer lifecycle). this is just cast raw pointer reference to &T.
/// the underlying pointer keeps remain.
/// This is useful when you want to cast raw pointer into concerate type but not need to release the underlying pointer for example calling rust object instance method from c ffi.
pub fn reference_from_boxed_pointer<'a, T>(ptr: *mut std::os::raw::c_void) -> &'a mut T {
    let obj = unsafe {Box::from_raw(ptr as *mut T)};
    unsafe {
        &mut *Box::into_raw(obj)
    }
}
