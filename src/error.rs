use std::error::Error;
use std::io::IoError;
use ffi;
use ffi::YamlErrorType;
use ffi::YamlErrorType::*;

#[deriving(Show, PartialEq, Copy)]
pub struct YamlMark {
    pub index: uint,
    pub line: uint,
    pub column: uint
}

impl YamlMark {
    pub fn conv(mark: &ffi::yaml_mark_t) -> YamlMark {
        YamlMark {
            index: mark.index as uint,
            line: mark.line as uint,
            column: mark.column as uint
        }
    }
}

#[deriving(Show, PartialEq)]
pub struct YamlErrorContext {
    pub byte_offset: uint,
    pub problem_mark: YamlMark,
    pub context: Option<String>,
    pub context_mark: YamlMark
}

#[deriving(Show, PartialEq)]
pub struct YamlError {
    pub kind: ffi::YamlErrorType,
    pub problem: Option<String>,
    pub io_error: Option<IoError>,
    pub context: Option<YamlErrorContext>
}

impl Error for YamlError {
    fn description(&self) -> &str {
        match self.kind {
            YAML_NO_ERROR => "No error is produced",
            YAML_MEMORY_ERROR => "Cannot allocate or reallocate a block of memory",
            YAML_READER_ERROR => "Cannot read or decode the input stream",
            YAML_SCANNER_ERROR => "Cannot scan the input stream",
            YAML_PARSER_ERROR => "Cannot parse the input stream",
            YAML_COMPOSER_ERROR => "Cannot compose a YAML document",
            YAML_WRITER_ERROR => "Cannot write to the output stream",
            YAML_EMITTER_ERROR => "Cannot emit a YAML stream",
        }
    }

    fn detail(&self) -> Option<String> {
        self.problem.clone()
    }

    fn cause(&self) -> Option<&Error> {
        match self.io_error {
            None => None,
            Some(ref e) => Some(e as &Error)
        }
    }
}
