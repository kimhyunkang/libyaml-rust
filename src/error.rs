use std::error::Error;
use ffi;
use ffi::YamlErrorType;
use ffi::YamlErrorType::*;

#[deriving(Show, PartialEq)]
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
pub enum YamlError {
    YamlParserError {
        kind: ffi::YamlErrorType,
        problem: Option<String>,
        byte_offset: uint,
        problem_mark: YamlMark,
        context: Option<String>,
        context_mark: YamlMark
    },

    YamlEmitterError {
        kind: ffi::YamlErrorType,
        problem: Option<String>
    }
}

impl YamlError {
    pub fn kind<'a>(&'a self) -> &'a ffi::YamlErrorType {
        match self {
            &YamlError::YamlParserError {
                kind: ref k,
                problem: _,
                byte_offset: _,
                problem_mark: _,
                context: _,
                context_mark: _
            } => k,
            &YamlError::YamlEmitterError {
                kind: ref k,
                problem: _,
            } => k,
        }
    }

    pub fn problem<'a>(&'a self) -> &'a Option<String> {
        match self {
            &YamlError::YamlParserError {
                kind: _,
                problem: ref p,
                byte_offset: _,
                problem_mark: _,
                context: _,
                context_mark: _
            } => p,
            &YamlError::YamlEmitterError {
                kind: _,
                problem: ref p,
            } => p,
        }
    }
}

impl Error for YamlError {
    fn description(&self) -> &str {
        match *self.kind() {
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
        self.problem().clone()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
