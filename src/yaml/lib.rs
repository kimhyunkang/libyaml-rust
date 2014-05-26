#![crate_id = "yaml#0.1-pre"]

#![comment = "LibYAML bindings for Rust"] 
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]
#![feature(phase)]

extern crate libc;

#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

use parser::YamlParser;
use constructor::{YamlStandardData, YamlStandardConstructor, YamlConstructor};
use std::result;

pub mod ffi;
pub mod event;
pub mod parser;
pub mod emitter;
pub mod document;
pub mod codecs;
pub mod constructor;

mod type_size;

pub fn version_string() -> String {
    let c_vsn = unsafe {
        std::c_str::CString::new(ffi::yaml_get_version_string(), false)
    };

    c_vsn.as_str().unwrap().to_owned()
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

pub fn parse_bytes(bytes: &[u8]) -> Result<Vec<YamlStandardData>, String> {
    let parser = parser::YamlByteParser::init(bytes);
    let ctor = YamlStandardConstructor::new();

    result::collect(parser.load().map(|doc_res| {
        match doc_res {
            Err(e) => Err(e.to_str()),
            Ok(doc) => ctor.construct(doc.root().unwrap())
        }
    }))
}

pub fn parse_io(reader: &mut Reader) -> Result<Vec<YamlStandardData>, String> {
    let parser = parser::YamlIoParser::init(reader);
    let ctor = YamlStandardConstructor::new();

    result::collect(parser.load().map(|doc_res| {
        match doc_res {
            Err(e) => Err(e.to_str()),
            Ok(doc) => ctor.construct(doc.root().unwrap())
        }
    }))
}

#[cfg(test)]
mod test {
    use std::mem;
    use std::io;
    use constructor::*;

    #[test]
    fn test_version_string() {
        let vsn = super::version_string();
        assert!("0.1.4".to_owned() <= vsn && vsn < "0.2".to_owned())
    }

    #[test]
    fn test_version() {
        let vsn = super::version();
        assert!((0, 1, 4) <= vsn && vsn < (0, 2, 0))
    }

    #[test]
    fn test_event_size() {
        assert_eq!(super::type_size::yaml_event_t_size, mem::size_of::<super::ffi::yaml_event_t>())
    }

    #[test]
    fn test_parser_size() {
        assert_eq!(super::type_size::yaml_parser_t_size, mem::size_of::<super::ffi::yaml_parser_t>())
    }

    #[test]
    fn test_emitter_size() {
        assert_eq!(super::type_size::yaml_emitter_t_size, mem::size_of::<super::ffi::yaml_emitter_t>())
    }

    #[test]
    fn test_document_size() {
        assert_eq!(super::type_size::yaml_document_t_size, mem::size_of::<super::ffi::yaml_document_t>())
    }

    #[test]
    fn test_node_size() {
        assert_eq!(super::type_size::yaml_node_t_size, mem::size_of::<super::ffi::yaml_node_t>())
    }

    #[test]
    fn test_parse_bytes() {
        let data = "[1, 2, 3]";
        assert_eq!(Ok(vec![YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])]), super::parse_bytes(data.as_bytes()))
    }

    #[test]
    fn test_parse_io() {
        let data = "[1, 2, 3]";
        let mut reader = io::BufReader::new(data.as_bytes());
        assert_eq!(Ok(vec![YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])]), super::parse_io(&mut reader))
    }
}
