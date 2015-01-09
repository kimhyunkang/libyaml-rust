use ffi;

use libc;

use std::slice;
use std::str;
use std::ptr;
use std::ffi::c_str_to_bytes;

pub fn decode_c_str(c_str: *const ffi::yaml_char_t) -> Option<String> {
    if c_str == ptr::null() {
        None
    } else {
        unsafe {
            let i8_str = c_str as *const i8;
            str::from_utf8(c_str_to_bytes(&i8_str)).map(|s| s.to_string()).ok()
        }
    }
}

pub fn decode_buf(buf: *const ffi::yaml_char_t, length: libc::size_t) -> Option<String> {
    if buf == ptr::null() {
        None
    } else {
        unsafe {
            str::from_utf8(slice::from_raw_buf(&buf, length as usize)).map(|s| { s.to_string() }).ok()
        }
    }
}

