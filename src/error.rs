use std::error::Error;
use std::io;
use std::fmt;
use ffi;
use ffi::YamlErrorType;
use ffi::YamlErrorType::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct YamlMark {
    pub index: usize,
    pub line: usize,
    pub column: usize
}

impl YamlMark {
    pub fn conv(mark: &ffi::yaml_mark_t) -> YamlMark {
        YamlMark {
            index: mark.index as usize,
            line: mark.line as usize,
            column: mark.column as usize
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct YamlErrorContext {
    pub byte_offset: usize,
    pub problem_mark: YamlMark,
    pub context: Option<String>,
    pub context_mark: YamlMark
}

#[derive(Debug)]
pub struct YamlError {
    pub kind: ffi::YamlErrorType,
    pub problem: Option<String>,
    pub io_error: Option<io::Error>,
    pub context: Option<YamlErrorContext>
}

impl PartialEq for YamlError {
    fn eq(&self, rhs: &YamlError) -> bool {
        self.kind == rhs.kind
            && self.problem == rhs.problem
            && self.io_error.is_none()
            && rhs.io_error.is_none()
            && self.context == rhs.context
    }
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

    fn cause(&self) -> Option<&Error> {
        match self.io_error {
            None => None,
            Some(ref e) => Some(e as &Error)
        }
    }
}

impl YamlError {
    pub fn new(kind: ffi::YamlErrorType, problem: Option<String>) -> YamlError {
        YamlError {
            kind: kind,
            problem: problem,
            io_error: None,
            context: None
        }
    }
}

impl fmt::Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.problem {
            None => return Ok(()),
            Some(ref s) => s.fmt(f)
        }
    }
}
