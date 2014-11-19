use ffi;

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
            &YamlParserError {
                kind: ref k,
                problem: _,
                byte_offset: _,
                problem_mark: _,
                context: _,
                context_mark: _
            } => k,
            &YamlEmitterError {
                kind: ref k,
                problem: _,
            } => k,
        }
    }

    pub fn problem<'a>(&'a self) -> &'a Option<String> {
        match self {
            &YamlParserError {
                kind: _,
                problem: ref p,
                byte_offset: _,
                problem_mark: _,
                context: _,
                context_mark: _
            } => p,
            &YamlEmitterError {
                kind: _,
                problem: ref p,
            } => p,
        }
    }
}
