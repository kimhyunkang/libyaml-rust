pub use type_size::*;
use libc::{c_char, c_uchar, c_int, c_void, size_t};
use parser::YamlIoParser;
use emitter::YamlEmitter;

#[allow(non_camel_case_types)]
pub type yaml_char_t = c_uchar;

#[allow(non_camel_case_types)]
pub type yaml_read_handler_t = extern fn(data: *mut YamlIoParser, buffer: *mut u8, size: size_t, size_read: *mut size_t) -> c_int;

#[allow(non_camel_case_types)]
pub type yaml_write_handler_t = extern fn(data: *mut YamlEmitter, buffer: *const u8, size: size_t) -> c_int;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum YamlErrorType {
    /** No error is produced. */
    YAML_NO_ERROR,

    /** Cannot allocate or reallocate a block of memory. */
    YAML_MEMORY_ERROR,

    /** Cannot read or decode the input stream. */
    YAML_READER_ERROR,
    /** Cannot scan the input stream. */
    YAML_SCANNER_ERROR,
    /** Cannot parse the input stream. */
    YAML_PARSER_ERROR,
    /** Cannot compose a YAML document. */
    YAML_COMPOSER_ERROR,

    /** Cannot write to the output stream. */
    YAML_WRITER_ERROR,
    /** Cannot emit a YAML stream. */
    YAML_EMITTER_ERROR
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub enum YamlSequenceStyle {
    /** Let the emitter choose the style. */
    YamlAnySequenceStyle = 0,

    /** The block sequence style. */
    YamlBlockSequenceStyle,
    /** The flow sequence style. */
    YamlFlowSequenceStyle
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
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

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_buffer_t {
    pub start: *const yaml_char_t,
    pub end: *const yaml_char_t,
    pub pointer: *const yaml_char_t,
    pub last: *const yaml_char_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_queue_t {
    pub start: *const c_void,
    pub end: *const c_void,
    pub head: *const c_void,
    pub tail: *const c_void
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_stack_t {
    pub start: *const c_void,
    pub end: *const c_void,
    pub top: *const c_void
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum yaml_node_type_t {
    /** An empty node. */
    YAML_NO_NODE = 0,

    /** A scalar node. */
    YAML_SCALAR_NODE,
    /** A sequence node. */
    YAML_SEQUENCE_NODE,
    /** A mapping node. */
    YAML_MAPPING_NODE
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_node_t {
    pub node_type: yaml_node_type_t,
    pub tag: *const yaml_char_t,
    pub data: yaml_node_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_scalar_node_t {
    pub value: *const yaml_char_t,
    pub length: size_t,
    pub style: YamlScalarStyle
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_sequence_node_t {
    pub items: yaml_stack_t,
    pub style: YamlSequenceStyle
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_node_pair_t {
    pub key: c_int,
    pub value: c_int
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_document_t {
    pub nodes: yaml_stack_t,

    pub version_directive: *const yaml_version_directive_t,
    pub tag_directives: yaml_tag_directive_list_t,

    pub start_implicit: c_int,
    pub end_implicit: c_int,

    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_parser_t {
    pub error: YamlErrorType,
    pub problem: *const c_char,
    pub problem_offset: size_t,
    pub problem_value: c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const c_char,
    pub context_mark: yaml_mark_t,

    pub read_handler: yaml_read_handler_t,
    pub read_handler_data: *const c_void,

    pub input: yaml_parser_input_t,
    pub eof: c_int,
    pub buffer: yaml_buffer_t,
    pub unread: size_t,
    pub raw_buffer: yaml_buffer_t,
    pub encoding: YamlEncoding,
    pub offset: size_t,
    pub mark: yaml_mark_t,

    pub stream_start_produced: c_int,
    pub stream_end_produced: c_int,
    pub flow_level: c_int,
    pub tokens: yaml_queue_t,
    pub tokens_parsed: size_t,
    pub token_available: c_int,

    pub indents: yaml_stack_t,
    pub indent: c_int,
    pub simple_key_allowed: c_int,
    pub simple_keys: yaml_stack_t,

    pub states: yaml_stack_t,
    pub parser_state: c_int,
    pub marks: yaml_stack_t,
    pub tag_directives: yaml_stack_t,
    pub aliases: yaml_stack_t,

    pub document: *const yaml_document_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum yaml_break_t {
    /** Let the parser choose the break type. */
    YAML_ANY_BREAK,
    /** Use CR for line breaks (Mac style). */
    YAML_CR_BREAK,
    /** Use LN for line breaks (Unix style). */
    YAML_LN_BREAK,
    /** Use CR LN for line breaks (DOS style). */
    YAML_CRLN_BREAK
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_emitter_t {
    pub error: YamlErrorType,
    pub problem: *const c_char,

    pub write_handler: yaml_write_handler_t,
    pub write_handler_data: *const c_void,

    pub output: yaml_emitter_output_t,
    pub buffer: yaml_buffer_t,
    pub raw_buffer: yaml_buffer_t,
    pub encoding: YamlEncoding,

    pub canonical: c_int,
    pub best_indent: c_int,
    pub best_width: c_int,
    pub unicode: c_int,
    pub line_break: yaml_break_t,

    pub states: yaml_stack_t,
    pub state: c_int,
    pub events: yaml_queue_t,
    pub indents: yaml_stack_t,
    pub tag_directives: yaml_stack_t,

    pub indent: c_int,

    pub flow_level: c_int,

    pub root_context: c_int,
    pub sequence_context: c_int,
    pub mapping_context: c_int,
    pub simple_key_context: c_int,

    pub line: c_int,
    pub column: c_int,
    pub whitespace: c_int,
    pub indention: c_int,
    pub open_ended: c_int,

    pub anchor_data: yaml_emitter_anchor_data_t,
    pub tag_data: yaml_emitter_tag_data_t,
    pub scalar_data: yaml_emitter_scalar_data_t,

    pub opened: c_int,
    pub closed: c_int,

    pub anchors: *const c_void,

    pub last_anchor_id: c_int,

    pub document: *const yaml_document_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_emitter_anchor_data_t {
    pub anchor: *const yaml_char_t,
    pub anchor_length: size_t,
    pub alias: c_int
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_emitter_tag_data_t {
    pub handle: *const yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *const yaml_char_t,
    pub suffix_length: size_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_emitter_scalar_data_t {
    pub value: *const yaml_char_t,
    pub length: size_t,
    pub multiline: c_int,
    pub flow_plain_allowed: c_int,
    pub block_plain_allowed: c_int,
    pub single_quoted_allowed: c_int,
    pub block_allowed: c_int,
    pub style: YamlScalarStyle,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_event_t {
    pub event_type: yaml_event_type_t,
    pub data: yaml_event_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_stream_start_event_t {
    pub encoding: YamlEncoding
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_list_t {
    pub start: *const yaml_tag_directive_t,
    pub end: *const yaml_tag_directive_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_document_start_event_t {
    pub version_directive: *const yaml_version_directive_t,
    pub tag_directives: yaml_tag_directive_list_t,
    pub implicit: c_int
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_document_end_event_t {
    pub implicit: c_int
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_alias_event_t {
    pub anchor: *const yaml_char_t
}

#[allow(non_camel_case_types)]
pub struct yaml_sequence_start_event_t {
    pub anchor: *const yaml_char_t,
    pub tag: *const yaml_char_t,
    pub implicit: c_int,
    pub style: YamlSequenceStyle
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_mapping_start_event_t {
    pub anchor: *const yaml_char_t,
    pub tag: *const yaml_char_t,
    pub implicit: c_int,
    pub style: YamlSequenceStyle
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_scalar_event_t {
    pub anchor: *const yaml_char_t,
    pub tag: *const yaml_char_t,
    pub value: *const yaml_char_t,
    pub length: size_t,
    pub plain_implicit: c_int,
    pub quoted_implicit: c_int,
    pub style: YamlScalarStyle
}

impl yaml_event_t {
    pub unsafe fn delete(&mut self) {
        yaml_event_delete(self);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_version_directive_t {
    pub major: c_int,
    pub minor: c_int
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_t {
    pub handle: *const c_char,
    pub prefix: *const c_char
}

#[link(name = "yaml")]
#[allow(improper_ctypes)]
extern {
    pub fn yaml_get_version_string() -> *const c_char;
    pub fn yaml_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_void;
    pub fn yaml_event_delete(event: *mut yaml_event_t) -> c_void;
    pub fn yaml_document_initialize(document: *mut yaml_document_t,
        version_directive: *const yaml_version_directive_t,
        tag_directives_start: *const yaml_tag_directive_t,
        tag_directives_end: *const yaml_tag_directive_t,
        start_implicit: c_int, end_implicit: c_int) -> c_int;
    pub fn yaml_document_get_node(document: *const yaml_document_t, index: c_int) -> *const yaml_node_t;
    pub fn yaml_document_get_root_node(document: *const yaml_document_t) -> *const yaml_node_t;
    pub fn yaml_document_delete(document: *mut yaml_document_t) -> c_void;
    pub fn yaml_document_add_scalar(document: *mut yaml_document_t,
        tag: *const yaml_char_t, value: *const yaml_char_t, length: c_int,
        style: YamlScalarStyle) -> c_int;
    pub fn yaml_document_add_sequence(document: *mut yaml_document_t,
        tag: *const yaml_char_t, style: YamlSequenceStyle) -> c_int;
    pub fn yaml_document_add_mapping(document: *mut yaml_document_t,
        tag: *const yaml_char_t, style: YamlSequenceStyle) -> c_int;
    pub fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int;
    pub fn yaml_parser_set_encoding(parser: *mut yaml_parser_t, encoding: YamlEncoding) -> c_void;
    pub fn yaml_parser_delete(parser: *mut yaml_parser_t) -> c_void;
    pub fn yaml_parser_set_input_string(parser: *mut yaml_parser_t, input: *const yaml_char_t, size: size_t) -> c_void;
    pub fn yaml_parser_set_input(parser: *mut yaml_parser_t, handler: yaml_read_handler_t, data: *const c_void) -> c_void;
    pub fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> c_int;
    pub fn yaml_parser_load(parser: *mut yaml_parser_t, document: *mut yaml_document_t) -> c_int;
    pub fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> c_int;
    pub fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> c_int;
    pub fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) -> c_void;
    pub fn yaml_emitter_set_output(emitter: *mut yaml_emitter_t, handler: yaml_write_handler_t, data: *const c_void) -> c_void;
    pub fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> c_int;
    pub fn yaml_stream_start_event_initialize(event: *mut yaml_event_t, encoding: YamlEncoding) -> c_int;
    pub fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    pub fn yaml_document_start_event_initialize(event: *mut yaml_event_t,
        version_directive: *const yaml_version_directive_t,
        tag_directives_start: *const yaml_tag_directive_t,
        tag_directies_end: *const yaml_tag_directive_t,
        implicit: c_int) -> c_int;
    pub fn yaml_document_end_event_initialize(event: *mut yaml_event_t, implicit: c_int) -> c_int;
    pub fn yaml_alias_event_initialize(event: *mut yaml_event_t, anchor: *const yaml_char_t) -> c_int;
    pub fn yaml_scalar_event_initialize(event: *mut yaml_event_t,
        anchor: *const yaml_char_t, tag: *const yaml_char_t,
        value: *const yaml_char_t, length: c_int,
        plain_implicit: c_int, quoted_implicit: c_int,
        style: YamlScalarStyle) -> c_int;
    pub fn yaml_sequence_start_event_initialize(event: *mut yaml_event_t,
        anchor: *const yaml_char_t, tag: *const yaml_char_t, implicit: c_int,
        style: YamlSequenceStyle) -> c_int;
    pub fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    pub fn yaml_mapping_start_event_initialize(event: *mut yaml_event_t,
        anchor: *const yaml_char_t, tag: *const yaml_char_t, implicit: c_int,
        style: YamlSequenceStyle) -> c_int;
    pub fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> c_int;
}
