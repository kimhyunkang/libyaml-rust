#![crate_id = "yaml#0.1-pre"]

#![comment = "LibYAML bindings for Rust"] 
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

use std::libc;

pub mod ffi;

pub fn version_string() -> ~str {
    let c_vsn = unsafe {
        std::c_str::CString::new(ffi::yaml_get_version_string(), false)
    };

    c_vsn.as_str().unwrap().into_owned()
}

pub fn version() -> (int, int, int) {
    let mut c_major: libc::c_int = 0;
    let mut c_minor: libc::c_int = 0;
    let mut c_patch: libc::c_int = 0;

    unsafe {
        ffi::yaml_get_version(
            &mut c_major as *mut libc::c_int,
            &mut c_minor as *mut libc::c_int,
            &mut c_patch as *mut libc::c_int
        );
    }

    (c_major as int, c_minor as int, c_patch as int)
}

mod test {
    #[test]
    fn test_version_string() {
        let vsn = super::version_string();
        assert!(~"0.1.4" <= vsn && vsn < ~"0.2")
    }

    #[test]
    fn test_version() {
        let vsn = super::version();
        assert!((0, 1, 4) <= vsn && vsn < (0, 2, 0))
    }
}
