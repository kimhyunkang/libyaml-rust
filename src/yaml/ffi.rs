pub use type_size::{yaml_parser_mem_t, new_yaml_parser_mem_t, yaml_event_data_t, new_yaml_event_data_t, yaml_event_type_t};
use std::libc::{c_char, c_uchar, c_int, c_void, size_t};

#[allow(non_camel_case_types)]
pub type yaml_char_t = c_uchar;

#[allow(non_camel_case_types)]
pub type yaml_read_handler_t = extern fn(data: *c_void, buffer: *c_uchar, size: size_t, size_read: *size_t) -> c_int;

/** An empty event. */
pub static YAML_NO_EVENT:yaml_event_type_t = 0;

/** A STREAM-START event. */
pub static YAML_STREAM_START_EVENT:yaml_event_type_t = 1;
/** A STREAM-END event. */
pub static YAML_STREAM_END_EVENT:yaml_event_type_t = 2;

/** A DOCUMENT-START event. */
pub static YAML_DOCUMENT_START_EVENT:yaml_event_type_t = 3;
/** A DOCUMENT-END event. */
pub static YAML_DOCUMENT_END_EVENT:yaml_event_type_t = 4;

/** An ALIAS event. */
pub static YAML_ALIAS_EVENT:yaml_event_type_t = 5;
/** A SCALAR event. */
pub static YAML_SCALAR_EVENT:yaml_event_type_t = 6;

/** A SEQUENCE-START event. */
pub static YAML_SEQUENCE_START_EVENT:yaml_event_type_t = 7;
/** A SEQUENCE-END event. */
pub static YAML_SEQUENCE_END_EVENT:yaml_event_type_t = 8;

/** A MAPPING-START event. */
pub static YAML_MAPPING_START_EVENT:yaml_event_type_t = 9;
/** A MAPPING-END event. */
pub static YAML_MAPPING_END_EVENT:yaml_event_type_t = 10;

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlSequenceStyle {
    /** Let the emitter choose the style. */
    YamlAnySequenceStyle = 0,

    /** The block sequence style. */
    YamlBlockSequenceStyle,
    /** The flow sequence style. */
    YamlFlowSequenceStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlScalarStyle {
    /** Let the emitter choose the style. */
    YamlAnyScalarStyle = 0,

    /** The plain scalar style. */
    YamlPlainScalarStyle,

    /** The single-quoted scalar style. */
    YamlSingleQuotedScalarStyle,
    /** The double-quoted scalar style. */
    YamlDoubleQuotedScalarStyle,

    /** The literal scalar style. */
    YamlLiteralScalarStyle,
    /** The folded scalar style. */
    YamlFoldedScalarStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlEncoding {
    /** Let the parser choose the encoding. */
    YamlAnyEncoding = 0,
    /** The default UTF-8 encoding. */
    YamlUtf8Encoding,
    /** The UTF-16-LE encoding with BOM. */
    YamlUtf16LeEncoding,
    /** The UTF-16-BE encoding with BOM. */
    YamlUtf16BeEncoding
}

#[allow(non_camel_case_types)]
pub struct yaml_mark_t {
    index: size_t,
    line: size_t,
    column: size_t
}

impl yaml_mark_t {
    pub fn new() -> yaml_mark_t {
        yaml_mark_t { index: 0, line: 0, column: 0 }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_parser_t {
    opaque: yaml_parser_mem_t
}

impl yaml_parser_t {
    pub fn new() -> yaml_parser_t {
        yaml_parser_t {
            opaque: new_yaml_parser_mem_t()
        }
    }

    pub unsafe fn initialize(&mut self) -> bool {
        yaml_parser_initialize(self) != 0
    }

    pub unsafe fn delete(&mut self) {
        yaml_parser_delete(self);
    }

    pub unsafe fn set_input_string(&mut self, input: *u8, size: uint) {
        yaml_parser_set_input_string(self, input, size as size_t);
    }

    pub unsafe fn parse(&mut self, event: &mut yaml_event_t) -> bool {
        yaml_parser_parse(self, event) != 0
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_event_t {
    event_type: yaml_event_type_t,
    data: yaml_event_data_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t
}

#[allow(non_camel_case_types)]
pub struct yaml_stream_start_event_t {
    encoding: YamlEncoding
}

#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_list_t {
    start: *yaml_tag_directive_t,
    end: *yaml_tag_directive_t,
}

#[allow(non_camel_case_types)]
pub struct yaml_document_start_event_t {
    version_directive: *yaml_version_directive_t,
    tag_directives: yaml_tag_directive_list_t,
    implicit: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_document_end_event_t {
    implicit: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_alias_event_t {
    anchor: *yaml_char_t
}

#[allow(non_camel_case_types)]
pub struct yaml_sequence_start_event_t {
    anchor: *yaml_char_t,
    tag: *yaml_char_t,
    implicit: c_int,
    style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_mapping_start_event_t {
    anchor: *yaml_char_t,
    tag: *yaml_char_t,
    implicit: c_int,
    style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_scalar_event_t {
    anchor: *yaml_char_t,
    tag: *yaml_char_t,
    value: *yaml_char_t,
    length: size_t,
    plain_implicit: c_int,
    quoted_implicit: c_int,
    style: YamlScalarStyle
}

impl yaml_event_t {
    pub fn new() -> yaml_event_t {
        yaml_event_t {
            event_type: YAML_NO_EVENT,
            data: new_yaml_event_data_t(),
            start_mark: yaml_mark_t::new(),
            end_mark: yaml_mark_t::new(),
        }
    }

    pub unsafe fn delete(&mut self) {
        yaml_event_delete(self);
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_version_directive_t {
    major: c_int,
    minor: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_t {
    handle: *c_char,
    prefix: *c_char
}

#[link(name = "yaml")]
extern {
    pub fn yaml_get_version_string() -> *c_char;
    pub fn yaml_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_void;
    pub fn yaml_event_delete(event: *mut yaml_event_t) -> c_void;
    pub fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int;
    pub fn yaml_parser_delete(parser: *mut yaml_parser_t) -> c_void;
    pub fn yaml_parser_set_input_string(parser: *mut yaml_parser_t, input: *yaml_char_t, size: size_t) -> c_void;
    pub fn yaml_parser_set_input(parser: *mut yaml_parser_t, handler: *yaml_read_handler_t, data: *c_void) -> c_void;
    pub fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> c_int;
}
