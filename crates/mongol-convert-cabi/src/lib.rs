//! C ABI for `mongol-convert`. The universal interop layer: PHP FFI, Go (cgo), Java (JNI/Panama),
//! Python (ctypes), Node (N-API/ffi), etc. all load this.
//!
//! Contract: UTF-8 in, UTF-8 out. `mongol_convert_translate` returns a heap string the caller must release
//! with `mongol_convert_free`, or NULL on any error. `mongol-convert` itself stays `#![forbid(unsafe_code)]`;
//! the unavoidable FFI `unsafe` is confined to this thin shim.

use mongol_convert::CodeType;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;

/// Translate `input` from encoding `from` to `to`. All three are NUL-terminated UTF-8 C strings;
/// `from`/`to` are canonical encoding names ("zvvnmod", "delehi", "menk_shape", "menk_letter",
/// "z52", "utn57"; PHP-style "menkshape" aliases are also accepted).
///
/// Returns a newly allocated UTF-8 C string (free it with [`mongol_convert_free`]), or NULL on error:
/// a NULL argument, invalid UTF-8, an unknown encoding name, or an unsupported conversion.
/// An empty (but valid) result is returned as a non-NULL empty string.
///
/// # Safety
/// `from`, `to`, `input` must each be NULL or a valid pointer to a NUL-terminated C string.
#[no_mangle]
pub extern "C" fn mongol_convert_translate(
    from: *const c_char,
    to: *const c_char,
    input: *const c_char,
) -> *mut c_char {
    if from.is_null() || to.is_null() || input.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null checked above; the caller guarantees NUL termination per the contract.
    let (from_c, to_c, input_c) =
        unsafe { (CStr::from_ptr(from), CStr::from_ptr(to), CStr::from_ptr(input)) };

    let translated = (|| -> Option<String> {
        let from = CodeType::from_str(from_c.to_str().ok()?).ok()?;
        let to = CodeType::from_str(to_c.to_str().ok()?).ok()?;
        mongol_convert::translate(from, to, input_c.to_str().ok()?).ok()
    })();

    match translated.and_then(|s| CString::new(s).ok()) {
        Some(c) => c.into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// Release a string returned by [`mongol_convert_translate`]. NULL is ignored.
///
/// # Safety
/// `ptr` must be NULL or a pointer previously returned by [`mongol_convert_translate`] and not yet freed.
#[no_mangle]
pub extern "C" fn mongol_convert_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: ptr came from CString::into_raw in mongol_convert_translate (checked non-null above).
    unsafe {
        drop(CString::from_raw(ptr));
    }
}

/// Library version, as a static NUL-terminated string. Do not free.
#[no_mangle]
pub extern "C" fn mongol_convert_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}
