use ffi;

use libc;

use std::str;
use std::ptr;
use std::c_str::CString;
use std::c_vec::CVec;

pub fn decode_c_str(c_str: *ffi::yaml_char_t) -> Option<String> {
    if c_str == ptr::null() {
        None
    } else {
        unsafe {
            CString::new(c_str as *i8, false).as_str().map(|s| { s.to_string() })
        }
    }
}

pub fn decode_buf(buf: *ffi::yaml_char_t, length: libc::size_t) -> Option<String> {
    if buf == ptr::null() {
        None
    } else {
        unsafe {
            str::from_utf8(CVec::new(buf as *mut u8, length as uint).as_slice()).map(|s| { s.to_string() })
        }
    }
}

