#![crate_name = "yaml"]
#![crate_type = "lib"]

extern crate libc;

extern crate regex;

use std::str;
use std::ffi::CStr;
use std::io::Read;

use parser::YamlParser;
use constructor::{YamlStandardData, YamlStandardConstructor, YamlConstructor};
use error::YamlError;

pub mod ffi;
pub mod error;
pub mod event;
pub mod parser;
pub mod emitter;
pub mod document;
pub mod codecs;
pub mod constructor;

mod type_size;

pub fn version_string() -> String {
    unsafe {
        str::from_utf8(CStr::from_ptr(ffi::yaml_get_version_string()).to_bytes()).unwrap().to_string()
    }
}

pub fn version() -> (isize, isize, isize) {
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

    (c_major as isize, c_minor as isize, c_patch as isize)
}

pub fn parse_bytes_utf8(bytes: &[u8]) -> Result<Vec<YamlStandardData>, YamlError> {
    parse_bytes(bytes, ffi::YamlEncoding::YamlUtf8Encoding)
}

pub fn parse_bytes(bytes: &[u8], encoding: ffi::YamlEncoding) -> Result<Vec<YamlStandardData>, YamlError> {
    let parser = parser::YamlByteParser::init(bytes, encoding);
    let ctor = YamlStandardConstructor::new();

    parser.load().map(|doc_res|
        doc_res.and_then(|doc| ctor.construct(doc.root().unwrap()))
    ).collect()
}

pub fn parse_io_utf8(reader: &mut Read) -> Result<Vec<YamlStandardData>, YamlError> {
    parse_io(reader, ffi::YamlEncoding::YamlUtf8Encoding)
}

pub fn parse_io(reader: &mut Read, encoding: ffi::YamlEncoding) -> Result<Vec<YamlStandardData>, YamlError> {
    let parser = parser::YamlIoParser::init(reader, encoding);
    let ctor = YamlStandardConstructor::new();

    parser.load().map(|doc_res|
        doc_res.and_then(|doc| ctor.construct(doc.root().unwrap()))
    ).collect()
}

#[cfg(test)]
mod test {
    use std::mem;
    use std::io::BufReader;
    use constructor::YamlStandardData::*;

    #[test]
    fn test_version_string() {
        let vsn = super::version_string();
        assert!("0.1.4".to_string() <= vsn && vsn < "0.3".to_string())
    }

    #[test]
    fn test_version() {
        let vsn = super::version();
        assert!((0, 1, 4) <= vsn && vsn < (0, 3, 0))
    }

    #[test]
    fn test_event_size() {
        assert_eq!(super::type_size::YAML_EVENT_T_SIZE, mem::size_of::<super::ffi::yaml_event_t>())
    }

    #[test]
    fn test_parser_size() {
        assert_eq!(super::type_size::YAML_PARSER_T_SIZE, mem::size_of::<super::ffi::yaml_parser_t>())
    }

    #[test]
    fn test_emitter_size() {
        assert_eq!(super::type_size::YAML_EMITTER_T_SIZE, mem::size_of::<super::ffi::yaml_emitter_t>())
    }

    #[test]
    fn test_document_size() {
        assert_eq!(super::type_size::YAML_DOCUMENT_T_SIZE, mem::size_of::<super::ffi::yaml_document_t>())
    }

    #[test]
    fn test_node_size() {
        assert_eq!(super::type_size::YAML_NODE_T_SIZE, mem::size_of::<super::ffi::yaml_node_t>())
    }

    #[test]
    fn test_parse_bytes() {
        let data = "[1, 2, 3]";
        assert_eq!(Ok(vec![YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])]), super::parse_bytes_utf8(data.as_bytes()))
    }

    #[test]
    fn test_parse_io() {
        let data = "[1, 2, 3]";
        let mut reader = BufReader::new(data.as_bytes());
        assert_eq!(Ok(vec![YamlSequence(vec![YamlInteger(1), YamlInteger(2), YamlInteger(3)])]), super::parse_io_utf8(&mut reader))
    }
}
