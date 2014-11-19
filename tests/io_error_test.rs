extern crate yaml;

use std::error::Error;
use std::io::{IoError, IoResult};
use std::io::IoErrorKind;

struct MockReader {
    _data: ()
}

impl MockReader {
    pub fn new() -> MockReader {
        MockReader { _data: () }
    }
}

impl Reader for MockReader {
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<uint> {
        Err(IoError {
            kind: IoErrorKind::OtherIoError,
            desc: "",
            detail: Some("mock reader".to_string())
        })
    }
}

#[test]
fn error_cause_test() {
    let mut mock_reader = MockReader::new();
    match yaml::parse_io_utf8(&mut mock_reader) {
        Ok(_) => panic!("Should return an error"),
        Err(e) => assert_eq!(e.cause().and_then(|ioe| ioe.detail()), Some("mock reader".to_string()))
    }
}
