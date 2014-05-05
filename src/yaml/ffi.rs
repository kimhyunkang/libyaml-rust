use std::libc::{c_char, c_uchar, c_int, c_void};

#[allow(non_camel_case_types)]
type yaml_char_t = c_uchar;

#[link(name = "yaml")]
extern {
    pub fn yaml_get_version_string() -> *c_char;
    pub fn yaml_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_void;
}
