pub use type_size::{yaml_parser_mem_t, new_yaml_parser_mem_t, yaml_event_data_t, new_yaml_event_data_t, yaml_event_type_t, yaml_error_type_t, yaml_parser_input_t, new_yaml_parser_input_t, yaml_node_type_t, yaml_node_data_t, new_yaml_node_data_t};
use std::libc::{c_char, c_uchar, c_int, c_void, size_t};
use std::ptr;
use parser::YamlIoParser;

#[allow(non_camel_case_types)]
pub type yaml_char_t = c_uchar;

#[allow(non_camel_case_types)]
pub type yaml_read_handler_t = extern fn(data: *mut YamlIoParser, buffer: *mut u8, size: size_t, size_read: *mut size_t) -> c_int;

#[allow(unused_variable)]
extern fn yaml_dummy_read_handler(data: *mut YamlIoParser, buffer: *mut u8, size: size_t, size_read: *mut size_t) -> c_int {
    return 0;
}

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

/** No error is produced. */
pub static YAML_NO_ERROR:yaml_error_type_t = 0;

/** Cannot allocate or reallocate a block of memory. */
pub static YAML_MEMORY_ERROR:yaml_error_type_t = 1;

/** Cannot read or decode the input stream. */
pub static YAML_READER_ERROR:yaml_error_type_t = 2;
/** Cannot scan the input stream. */
pub static YAML_SCANNER_ERROR:yaml_error_type_t = 3;
/** Cannot parse the input stream. */
pub static YAML_PARSER_ERROR:yaml_error_type_t = 4;
/** Cannot compose a YAML document. */
pub static YAML_COMPOSER_ERROR:yaml_error_type_t = 5;

/** Cannot write to the output stream. */
pub static YAML_WRITER_ERROR:yaml_error_type_t = 6;
/** Cannot emit a YAML stream. */
pub static YAML_EMITTER_ERROR:yaml_error_type_t = 7;

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
pub struct yaml_buffer_t {
    start: *yaml_char_t,
    end: *yaml_char_t,
    pointer: *yaml_char_t,
    last: *yaml_char_t
}

impl yaml_buffer_t {
    fn new() -> yaml_buffer_t {
        yaml_buffer_t {
            start: ptr::null(),
            end: ptr::null(),
            pointer: ptr::null(),
            last: ptr::null(),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_queue_t {
    start: *c_void,
    end: *c_void,
    head: *c_void,
    tail: *c_void
}

impl yaml_queue_t {
    fn new() -> yaml_queue_t {
        yaml_queue_t {
            start: ptr::null(),
            end: ptr::null(),
            head: ptr::null(),
            tail: ptr::null(),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_stack_t {
    start: *c_void,
    end: *c_void,
    top: *c_void
}

impl yaml_stack_t {
    fn new() -> yaml_stack_t {
        yaml_stack_t {
            start: ptr::null(),
            end: ptr::null(),
            top: ptr::null()
        }
    }
}

/** An empty node. */
pub static YAML_NO_NODE:yaml_node_type_t = 0;

/** A scalar node. */
pub static YAML_SCALAR_NODE:yaml_node_type_t = 1;
/** A sequence node. */
pub static YAML_SEQUENCE_NODE:yaml_node_type_t = 2;
/** A mapping node. */
pub static YAML_MAPPING_NODE:yaml_node_type_t = 3;

#[allow(non_camel_case_types)]
pub struct yaml_node_t {
    node_type: yaml_node_type_t,
    tag: *yaml_char_t,
    data: yaml_node_data_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

impl yaml_node_t {
    pub fn new() -> yaml_node_t {
        yaml_node_t {
            node_type: YAML_NO_NODE,
            tag: ptr::null(),
            data: new_yaml_node_data_t(),
            start_mark: yaml_mark_t::new(),
            end_mark: yaml_mark_t::new()
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_scalar_node_t {
    value: *yaml_char_t,
    length: size_t,
    style: YamlScalarStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_sequence_node_t {
    items: yaml_stack_t,
    style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_node_pair_t {
    key: c_int,
    value: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_document_t {
    nodes: yaml_stack_t,

    version_directive: *yaml_version_directive_t,
    tag_directives: yaml_tag_directive_list_t,

    start_implicit: c_int,
    end_implicit: c_int,

    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

impl yaml_document_t {
    pub fn new() -> yaml_document_t {
        yaml_document_t {
            nodes: yaml_stack_t::new(),
            version_directive: ptr::null(),
            tag_directives: yaml_tag_directive_list_t { start: ptr::null(), end: ptr::null() },
            start_implicit: 0,
            end_implicit: 0,
            start_mark: yaml_mark_t::new(),
            end_mark: yaml_mark_t::new()
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_parser_t {
    error: yaml_error_type_t,
    problem: *c_char,
    problem_offset: size_t,
    problem_value: c_int,
    problem_mark: yaml_mark_t,
    context: *c_char,
    context_mark: yaml_mark_t,

    read_handler: yaml_read_handler_t,
    read_handler_data: *c_void,

    input: yaml_parser_input_t,
    eof: c_int,
    buffer: yaml_buffer_t,
    unread: size_t,
    raw_buffer: yaml_buffer_t,
    encoding: YamlEncoding,
    offset: size_t,
    mark: yaml_mark_t,

    stream_start_produced: c_int,
    stream_end_produced: c_int,
    flow_level: c_int,
    tokens: yaml_queue_t,
    tokens_parsed: size_t,
    token_available: c_int,

    indents: yaml_stack_t,
    indent: c_int,
    simple_key_allowed: c_int,
    simple_keys: yaml_stack_t,

    states: yaml_stack_t,
    parser_state: c_int,
    marks: yaml_stack_t,
    tag_directives: yaml_stack_t,
    aliases: yaml_stack_t,

    document: *yaml_document_t,
}

impl yaml_parser_t {
    pub fn new() -> yaml_parser_t {
        yaml_parser_t {
            error: 0,
            problem: ptr::null(),
            problem_offset: 0,
            problem_value: 0,
            problem_mark: yaml_mark_t::new(),
            context: ptr::null(),
            context_mark: yaml_mark_t::new(),

            read_handler: yaml_dummy_read_handler,
            read_handler_data: ptr::null(),

            input: new_yaml_parser_input_t(),
            eof: 0,
            buffer: yaml_buffer_t::new(),
            unread: 0,
            raw_buffer: yaml_buffer_t::new(),
            encoding: YamlAnyEncoding,
            offset: 0,
            mark: yaml_mark_t::new(),

            stream_start_produced: 0,
            stream_end_produced: 0,
            flow_level: 0,
            tokens: yaml_queue_t::new(),
            tokens_parsed: 0,
            token_available: 0,

            indents: yaml_stack_t::new(),
            indent: 0,
            simple_key_allowed: 0,
            simple_keys: yaml_stack_t::new(),

            states: yaml_stack_t::new(),
            parser_state: 0,
            marks: yaml_stack_t::new(),
            tag_directives: yaml_stack_t::new(),
            aliases: yaml_stack_t::new(),

            document: ptr::null()
        }
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
    pub fn yaml_document_get_node(document: *yaml_document_t, index: c_int) -> *yaml_node_t;
    pub fn yaml_document_get_root_node(document: *yaml_document_t) -> *yaml_node_t;
    pub fn yaml_document_delete(document: *mut yaml_document_t) -> c_void;
    pub fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int;
    pub fn yaml_parser_delete(parser: *mut yaml_parser_t) -> c_void;
    pub fn yaml_parser_set_input_string(parser: *mut yaml_parser_t, input: *yaml_char_t, size: size_t) -> c_void;
    pub fn yaml_parser_set_input(parser: *mut yaml_parser_t, handler: yaml_read_handler_t, data: *c_void) -> c_void;
    pub fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> c_int;
    pub fn yaml_parser_load(parser: *mut yaml_parser_t, document: *mut yaml_document_t) -> c_int;
}
