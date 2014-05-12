pub use type_size::*;
use libc::{c_char, c_uchar, c_int, c_void, size_t};
use std::ptr;
use std::cast;
use parser::YamlIoParser;
use emitter::YamlEmitter;

#[allow(non_camel_case_types)]
pub type yaml_char_t = c_uchar;

#[allow(non_camel_case_types)]
pub type yaml_read_handler_t = extern fn(data: *mut YamlIoParser, buffer: *mut u8, size: size_t, size_read: *mut size_t) -> c_int;

#[allow(non_camel_case_types)]
pub type yaml_write_handler_t = extern fn(data: *mut YamlEmitter, buffer: *u8, size: size_t) -> c_int;

#[repr(C)]
#[deriving(Show)]
#[deriving(Eq)]
pub enum YamlErrorType {
    /** No error is produced. */
    YamlNoError,

    /** Cannot allocate or reallocate a block of memory. */
    YamlMemoryError,

    /** Cannot read or decode the input stream. */
    YamlReaderError,
    /** Cannot scan the input stream. */
    YamlScannerError,
    /** Cannot parse the input stream. */
    YamlParserError,
    /** Cannot compose a YAML document. */
    YamlComposerError,

    /** Cannot write to the output stream. */
    YamlWriterError,
    /** Cannot emit a YAML stream. */
    YamlEmitterError
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

#[deriving(Eq)]
#[deriving(Show)]
#[repr(C)]
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

#[deriving(Eq)]
#[deriving(Show)]
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

#[allow(non_camel_case_types)]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t
}

impl yaml_mark_t {
    pub fn new() -> yaml_mark_t {
        yaml_mark_t { index: 0, line: 0, column: 0 }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_buffer_t {
    pub start: *yaml_char_t,
    pub end: *yaml_char_t,
    pub pointer: *yaml_char_t,
    pub last: *yaml_char_t
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
    pub start: *c_void,
    pub end: *c_void,
    pub head: *c_void,
    pub tail: *c_void
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
    pub start: *c_void,
    pub end: *c_void,
    pub top: *c_void
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
    pub node_type: yaml_node_type_t,
    pub tag: *yaml_char_t,
    pub data: yaml_node_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub value: *yaml_char_t,
    pub length: size_t,
    pub style: YamlScalarStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_sequence_node_t {
    pub items: yaml_stack_t,
    pub style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_node_pair_t {
    pub key: c_int,
    pub value: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_document_t {
    pub nodes: yaml_stack_t,

    pub version_directive: *yaml_version_directive_t,
    pub tag_directives: yaml_tag_directive_list_t,

    pub start_implicit: c_int,
    pub end_implicit: c_int,

    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub error: YamlErrorType,
    pub problem: *c_char,
    pub problem_offset: size_t,
    pub problem_value: c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *c_char,
    pub context_mark: yaml_mark_t,

    pub read_handler: yaml_read_handler_t,
    pub read_handler_data: *c_void,

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

    pub document: *yaml_document_t,
}

impl yaml_parser_t {
    pub fn new() -> yaml_parser_t {
        yaml_parser_t {
            error: YamlNoError,
            problem: ptr::null(),
            problem_offset: 0,
            problem_value: 0,
            problem_mark: yaml_mark_t::new(),
            context: ptr::null(),
            context_mark: yaml_mark_t::new(),

            read_handler: unsafe { cast::transmute(0) },
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

#[allow(non_camel_case_types)]
pub struct yaml_emitter_t {
    pub error: YamlErrorType,
    pub problem: *c_char,

    pub write_handler: yaml_write_handler_t,
    pub write_handler_data: *c_void,

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

    pub anchors: *c_void,

    pub last_anchor_id: c_int,

    pub document: *yaml_document_t
}

impl yaml_emitter_t {
    pub fn new() -> yaml_emitter_t {
        yaml_emitter_t {
            error: YamlNoError,
            problem: ptr::null(),

            write_handler: unsafe { cast::transmute(0) },
            write_handler_data: ptr::null(),

            output: new_yaml_emitter_output_t(),
            buffer: yaml_buffer_t::new(),
            raw_buffer: yaml_buffer_t::new(),
            encoding: YamlAnyEncoding,

            canonical: 0,
            best_indent: 0,
            best_width: 0,
            unicode: 0,
            line_break: YAML_ANY_BREAK,

            states: yaml_stack_t::new(),
            state: 0,
            events: yaml_queue_t::new(),
            indents: yaml_stack_t::new(),
            tag_directives: yaml_stack_t::new(),

            indent: 0,

            flow_level: 0,

            root_context: 0,
            sequence_context: 0,
            mapping_context: 0,
            simple_key_context: 0,

            line: 0,
            column: 0,
            whitespace: 0,
            indention: 0,
            open_ended: 0,

            anchor_data: yaml_emitter_anchor_data_t::new(),
            tag_data: yaml_emitter_tag_data_t::new(),
            scalar_data: yaml_emitter_scalar_data_t::new(),

            opened: 0,
            closed: 0,

            anchors: ptr::null(),

            last_anchor_id: 0,

            document: ptr::null()
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_emitter_anchor_data_t {
    pub anchor: *yaml_char_t,
    pub anchor_length: size_t,
    pub alias: c_int
}

impl yaml_emitter_anchor_data_t {
    pub fn new() -> yaml_emitter_anchor_data_t {
        yaml_emitter_anchor_data_t {
            anchor: ptr::null(),
            anchor_length: 0,
            alias: 0
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_emitter_tag_data_t {
    pub handle: *yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *yaml_char_t,
    pub suffix_length: size_t
}

impl yaml_emitter_tag_data_t {
    pub fn new() -> yaml_emitter_tag_data_t {
        yaml_emitter_tag_data_t {
            handle: ptr::null(),
            handle_length: 0,
            suffix: ptr::null(),
            suffix_length: 0
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_emitter_scalar_data_t {
    pub value: *yaml_char_t,
    pub length: size_t,
    pub multiline: c_int,
    pub flow_plain_allowed: c_int,
    pub block_plain_allowed: c_int,
    pub single_quoted_allowed: c_int,
    pub block_allowed: c_int,
    pub style: YamlScalarStyle,
}

impl yaml_emitter_scalar_data_t {
    pub fn new() -> yaml_emitter_scalar_data_t {
        yaml_emitter_scalar_data_t {
            value: ptr::null(),
            length: 0,
            multiline: 0,
            flow_plain_allowed: 0,
            block_plain_allowed: 0,
            single_quoted_allowed: 0,
            block_allowed: 0,
            style: YamlAnyScalarStyle,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct yaml_event_t {
    pub event_type: yaml_event_type_t,
    pub data: yaml_event_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t
}

#[allow(non_camel_case_types)]
pub struct yaml_stream_start_event_t {
    pub encoding: YamlEncoding
}

#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_list_t {
    pub start: *yaml_tag_directive_t,
    pub end: *yaml_tag_directive_t,
}

#[allow(non_camel_case_types)]
pub struct yaml_document_start_event_t {
    pub version_directive: *yaml_version_directive_t,
    pub tag_directives: yaml_tag_directive_list_t,
    pub implicit: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_document_end_event_t {
    pub implicit: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_alias_event_t {
    pub anchor: *yaml_char_t
}

#[allow(non_camel_case_types)]
pub struct yaml_sequence_start_event_t {
    pub anchor: *yaml_char_t,
    pub tag: *yaml_char_t,
    pub implicit: c_int,
    pub style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_mapping_start_event_t {
    pub anchor: *yaml_char_t,
    pub tag: *yaml_char_t,
    pub implicit: c_int,
    pub style: YamlSequenceStyle
}

#[allow(non_camel_case_types)]
pub struct yaml_scalar_event_t {
    pub anchor: *yaml_char_t,
    pub tag: *yaml_char_t,
    pub value: *yaml_char_t,
    pub length: size_t,
    pub plain_implicit: c_int,
    pub quoted_implicit: c_int,
    pub style: YamlScalarStyle
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
    pub major: c_int,
    pub minor: c_int
}

#[allow(non_camel_case_types)]
pub struct yaml_tag_directive_t {
    pub handle: *c_char,
    pub prefix: *c_char
}

#[link(name = "yaml")]
extern {
    pub fn yaml_get_version_string() -> *c_char;
    pub fn yaml_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_void;
    pub fn yaml_event_delete(event: *mut yaml_event_t) -> c_void;
    pub fn yaml_document_initialize(document: *mut yaml_document_t,
        version_directive: *yaml_version_directive_t,
        tag_directives_start: *yaml_tag_directive_t,
        tag_directives_end: *yaml_tag_directive_t,
        start_implicit: c_int, end_implicit: c_int) -> c_int;
    pub fn yaml_document_get_node(document: *yaml_document_t, index: c_int) -> *yaml_node_t;
    pub fn yaml_document_get_root_node(document: *yaml_document_t) -> *yaml_node_t;
    pub fn yaml_document_delete(document: *mut yaml_document_t) -> c_void;
    pub fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int;
    pub fn yaml_parser_delete(parser: *mut yaml_parser_t) -> c_void;
    pub fn yaml_parser_set_input_string(parser: *mut yaml_parser_t, input: *yaml_char_t, size: size_t) -> c_void;
    pub fn yaml_parser_set_input(parser: *mut yaml_parser_t, handler: yaml_read_handler_t, data: *c_void) -> c_void;
    pub fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> c_int;
    pub fn yaml_parser_load(parser: *mut yaml_parser_t, document: *mut yaml_document_t) -> c_int;
    pub fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> c_int;
    pub fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> c_int;
    pub fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) -> c_void;
    pub fn yaml_emitter_set_output(emitter: *mut yaml_emitter_t, handler: yaml_write_handler_t, data: *c_void) -> c_void;
    pub fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> c_int;
    pub fn yaml_stream_start_event_initialize(event: *mut yaml_event_t, encoding: YamlEncoding) -> c_int;
    pub fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    pub fn yaml_document_start_event_initialize(event: *mut yaml_event_t,
        version_directive: *yaml_version_directive_t,
        tag_directives_start: *yaml_tag_directive_t,
        tag_directies_end: *yaml_tag_directive_t,
        implicit: c_int) -> c_int;
    pub fn yaml_document_end_event_initialize(event: *mut yaml_event_t, implicit: c_int) -> c_int;
    pub fn yaml_alias_event_initialize(event: *mut yaml_event_t, anchor: *yaml_char_t) -> c_int;
    pub fn yaml_scalar_event_initialize(event: *mut yaml_event_t,
        anchor: *yaml_char_t, tag: *yaml_char_t,
        value: *yaml_char_t, length: c_int,
        plain_implicit: c_int, quoted_implicit: c_int,
        style: YamlScalarStyle) -> c_int;
    pub fn yaml_sequence_start_event_initialize(event: *mut yaml_event_t,
        anchor: *yaml_char_t, tag: *yaml_char_t, implicit: c_int,
        style: YamlSequenceStyle) -> c_int;
    pub fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> c_int;
    pub fn yaml_mapping_start_event_initialize(event: *mut yaml_event_t,
        anchor: *yaml_char_t, tag: *yaml_char_t, implicit: c_int,
        style: YamlSequenceStyle) -> c_int;
    pub fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> c_int;
}
