use ffi;
pub use ffi::{YamlEncoding, YamlUtf8Encoding, YamlUtf16BeEncoding, YamlUtf16LeEncoding, YamlScalarStyle, YamlSequenceStyle};
use std::cast;
use std::ptr;
use std::libc;
use std::io;
use std::c_str::CString;
use std::c_vec::CVec;

fn c_str_into_owned_bytes(c_str: *ffi::yaml_char_t) -> Option<~[u8]> {
    unsafe {
        if c_str == ptr::null() {
            None
        } else {
            Some(CString::new(c_str as *i8, false).as_bytes_no_nul().into_owned())
        }
    }
}

pub struct InternalEvent {
    pub event_mem: ffi::yaml_event_t
}

impl Drop for InternalEvent {
    fn drop(&mut self) {
        unsafe {
            self.event_mem.delete()
        }
    }
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlVersionDirective {
    pub major: int,
    pub minor: int,
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlTagDirective {
    pub handle: ~[u8],
    pub prefix: ~[u8],
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlSequenceParam {
    anchor: Option<~[u8]>,
    tag: Option<~[u8]>,
    implicit: bool,
    style: YamlSequenceStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlScalarParam {
    anchor: Option<~[u8]>,
    tag: Option<~[u8]>,
    value: ~[u8],
    plain_implicit: bool,
    quoted_implicit: bool,
    style: YamlScalarStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlErrorType {
    YamlNoError,
    YamlMemoryError,
    YamlReaderError,
    YamlScannerError,
    YamlParserError,
    YamlComposerError,
    YamlWriterError,
    YamlEmitterError,
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlMark {
    index: uint,
    line: uint,
    column: uint
}

impl YamlMark {
    fn conv(mark: &ffi::yaml_mark_t) -> YamlMark {
        YamlMark {
            index: mark.index as uint,
            line: mark.line as uint,
            column: mark.column as uint
        }
    }
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlError {
    kind: YamlErrorType,
    problem: Option<~str>,
    byte_offset: uint,
    problem_mark: YamlMark,
    context: Option<~str>,
    context_mark: YamlMark,
}

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlEvent {
    YamlNoEvent,
    YamlStreamStartEvent(YamlEncoding),
    YamlStreamEndEvent,
    YamlDocumentStartEvent(Option<YamlVersionDirective>, ~[YamlTagDirective], bool),
    YamlDocumentEndEvent(bool),
    YamlAliasEvent(~[u8]),
    YamlScalarEvent(YamlScalarParam),
    YamlSequenceStartEvent(YamlSequenceParam),
    YamlSequenceEndEvent,
    YamlMappingStartEvent(YamlSequenceParam),
    YamlMappingEndEvent,
}

pub struct YamlEventStream<P> {
    parser: ~P,
}

impl<P:YamlParser> YamlEventStream<P> {
    fn next_event(&mut self) -> Result<YamlEvent, YamlError> {
        unsafe {
            match self.parser.parse_event() {
                Some(evt) => Ok(evt),
                None => Err(self.parser.base_parser_ref().get_error())
            }
        }
    }
}

pub trait YamlParser {
    unsafe fn base_parser_ref<'r>(&'r mut self) -> &'r mut YamlBaseParser;

    unsafe fn parse_event(&mut self) -> Option<YamlEvent> {
        let mut event = InternalEvent {
            event_mem: ffi::yaml_event_t::new()
        };

        if !self.base_parser_ref().parse(&mut event.event_mem) {
            None
        } else {
            Some(match event.event_mem.event_type {
                ffi::YAML_NO_EVENT => YamlNoEvent,
                ffi::YAML_STREAM_START_EVENT => {
                    let evt_data: &ffi::yaml_stream_start_event_t = cast::transmute(&event.event_mem.data);
                    YamlStreamStartEvent(evt_data.encoding)
                },
                ffi::YAML_STREAM_END_EVENT => YamlStreamEndEvent,
                ffi::YAML_DOCUMENT_START_EVENT => {
                    let evt_data: &ffi::yaml_document_start_event_t = cast::transmute(&event.event_mem.data);
                    let vsn_dir = if evt_data.version_directive == ptr::null() {
                        None
                    } else {
                        let c_vsn_dir: &ffi::yaml_version_directive_t = cast::transmute(evt_data.version_directive);
                        Some(YamlVersionDirective { major: c_vsn_dir.major as int, minor: c_vsn_dir.minor as int })
                    };
                    let mut tag_dirs = ~[];
                    let mut tag_ptr = evt_data.tag_directives.start;
                    while tag_ptr != ptr::null() && tag_ptr != evt_data.tag_directives.end {
                        let tag_ref: &ffi::yaml_tag_directive_t = cast::transmute(tag_ptr);
                        let handle = CString::new(tag_ref.handle, false).as_bytes_no_nul().into_owned();
                        let prefix = CString::new(tag_ref.prefix, false).as_bytes_no_nul().into_owned();
                        tag_dirs.push(YamlTagDirective { handle: handle, prefix: prefix });
                        tag_ptr = tag_ptr.offset(1);
                    }
                    let implicit = evt_data.implicit != 0;

                    YamlDocumentStartEvent(vsn_dir, tag_dirs, implicit)
                },
                ffi::YAML_DOCUMENT_END_EVENT => {
                    let evt_data: &ffi::yaml_document_end_event_t = cast::transmute(&event.event_mem.data);
                    let implicit = evt_data.implicit != 0;

                    YamlDocumentEndEvent(implicit)
                },
                ffi::YAML_ALIAS_EVENT => {
                    let evt_data: &ffi::yaml_alias_event_t = cast::transmute(&event.event_mem.data);
                    let anchor = CString::new(evt_data.anchor as *i8, false).as_bytes_no_nul().into_owned();

                    YamlAliasEvent(anchor)
                },
                ffi::YAML_SCALAR_EVENT => {
                    let evt_data: &ffi::yaml_scalar_event_t = cast::transmute(&event.event_mem.data);
                    let value = CVec::new(evt_data.value as *mut u8, evt_data.length as uint).as_slice().into_owned();

                    YamlScalarEvent(YamlScalarParam {
                        anchor: c_str_into_owned_bytes(evt_data.anchor),
                        tag: c_str_into_owned_bytes(evt_data.tag),
                        value: value,
                        plain_implicit: evt_data.plain_implicit != 0,
                        quoted_implicit: evt_data.quoted_implicit != 0,
                        style: evt_data.style
                    })
                },
                ffi::YAML_SEQUENCE_START_EVENT => {
                    let evt_data: &ffi::yaml_sequence_start_event_t = cast::transmute(&event.event_mem.data);

                    YamlSequenceStartEvent(YamlSequenceParam {
                        anchor: c_str_into_owned_bytes(evt_data.anchor),
                        tag: c_str_into_owned_bytes(evt_data.tag),
                        implicit: evt_data.implicit != 0,
                        style: evt_data.style
                    })
                },
                ffi::YAML_SEQUENCE_END_EVENT => YamlSequenceEndEvent,
                ffi::YAML_MAPPING_START_EVENT => {
                    let evt_data: &ffi::yaml_mapping_start_event_t = cast::transmute(&event.event_mem.data);

                    YamlMappingStartEvent(YamlSequenceParam {
                        anchor: c_str_into_owned_bytes(evt_data.anchor),
                        tag: c_str_into_owned_bytes(evt_data.tag),
                        implicit: evt_data.implicit != 0,
                        style: evt_data.style
                    })
                },
                ffi::YAML_MAPPING_END_EVENT => YamlMappingEndEvent,
                _ => fail!("unknown event type")
            })
        }
    }

    fn parse(~self) -> YamlEventStream<Self> {
        YamlEventStream {
            parser: self,
        }
    }
}

extern fn handle_reader_cb(data: *mut YamlIoParser, buffer: *mut u8, size: libc::size_t, size_read: *mut libc::size_t) -> libc::c_int {
    unsafe {
        let mut buf = CVec::new(buffer, size as uint);
        let parser = &mut *data;
        match parser.reader.read(buf.as_mut_slice()) {
            Ok(size) => {
                *size_read = size as libc::size_t;
                return 1;
            },
            Err(err) => {
                match err.kind {
                    io::EndOfFile => {
                        *size_read = 0;
                        return 1;
                    },
                    _ => {
                        return 0;
                    }
                }
            }
        }
    }
}

pub struct YamlBaseParser {
    parser_mem: ffi::yaml_parser_t,
}

impl YamlBaseParser {
    fn new() -> YamlBaseParser {
        YamlBaseParser {
            parser_mem: ffi::yaml_parser_t::new()
        }
    }

    unsafe fn initialize(&mut self) -> bool {
        ffi::yaml_parser_initialize(&mut self.parser_mem) != 0
    }

    unsafe fn set_input_string(&mut self, input: *u8, size: uint) {
        ffi::yaml_parser_set_input_string(&mut self.parser_mem, input, size as libc::size_t);
    }

    unsafe fn parse(&mut self, event: &mut ffi::yaml_event_t) -> bool {
        ffi::yaml_parser_parse(&mut self.parser_mem, event) != 0
    }

    unsafe fn get_error(&self) -> YamlError {
        let kind = match self.parser_mem.error {
            ffi::YAML_NO_ERROR => YamlNoError,
            ffi::YAML_READER_ERROR => YamlReaderError,
            ffi::YAML_SCANNER_ERROR => YamlScannerError,
            ffi::YAML_PARSER_ERROR => YamlParserError,
            ffi::YAML_COMPOSER_ERROR => YamlComposerError,
            ffi::YAML_WRITER_ERROR => YamlWriterError,
            ffi::YAML_EMITTER_ERROR => YamlEmitterError,
            _ => fail!("unknown error type")
        };

        YamlError {
            kind: kind,
            problem: CString::new(self.parser_mem.problem, false).as_str().map(|s| s.into_owned()),
            byte_offset: self.parser_mem.problem_offset as uint,
            problem_mark: YamlMark::conv(&self.parser_mem.problem_mark),
            context: CString::new(self.parser_mem.problem, false).as_str().map(|s| s.into_owned()),
            context_mark: YamlMark::conv(&self.parser_mem.context_mark),
        }
    }
}

impl Drop for YamlBaseParser {
    fn drop(&mut self) {
        unsafe {
            ffi::yaml_parser_delete(&mut self.parser_mem);
        }
    }
}

pub struct YamlByteParser<'r> {
    base_parser: YamlBaseParser
}

impl<'r> YamlParser for YamlByteParser<'r> {
    unsafe fn base_parser_ref<'r>(&'r mut self) -> &'r mut YamlBaseParser {
        &mut self.base_parser
    }
}

impl<'r> YamlByteParser<'r> {
    pub fn init(bytes: &'r [u8]) -> ~YamlByteParser<'r> {
        let mut parser = ~YamlByteParser {
            base_parser: YamlBaseParser::new()
        };

        unsafe {
            if !parser.base_parser.initialize() {
                fail!("failed to initialize yaml_parser_t");
            }
            parser.base_parser.set_input_string(bytes.as_ptr(), bytes.len());
        }

        parser
    }
}

pub struct YamlIoParser {
    base_parser: YamlBaseParser,
    reader: ~Reader,
}

impl<'r> YamlParser for YamlIoParser {
    unsafe fn base_parser_ref<'r>(&'r mut self) -> &'r mut YamlBaseParser {
        &mut self.base_parser
    }
}

impl YamlIoParser {
    pub fn init(reader: ~Reader) -> ~YamlIoParser {
        let mut parser = ~YamlIoParser {
            base_parser: YamlBaseParser::new(),
            reader: reader
        };

        unsafe {
            if !parser.base_parser.initialize() {
                fail!("failed to initialize yaml_parser_t");
            }

            ffi::yaml_parser_set_input(&mut parser.base_parser.parser_mem, handle_reader_cb, cast::transmute(&mut *parser));
        }

        parser
    }
} 

#[test]
fn test_byte_parser() {
    let data = "[1, 2, 3]";
    let parser = YamlByteParser::init(data.as_bytes());
    let expected = ~[
        YamlStreamStartEvent(ffi::YamlUtf8Encoding),
        YamlDocumentStartEvent(None, ~[], true),
        YamlSequenceStartEvent(YamlSequenceParam{anchor: None, tag: None, implicit: true, style: ffi::YamlFlowSequenceStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[49u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[50u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[51u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlSequenceEndEvent,
        YamlDocumentEndEvent(true),
        YamlStreamEndEvent
    ];

    let mut produced = ~[];
    let mut stream = parser.parse();

    loop {
        match stream.next_event() {
            Ok(YamlNoEvent) => {
                break;
            },
            Ok(evt) => {
                produced.push(evt);
            },
            Err(err) => {
                fail!("{:?}", err);
            }
        }
    }

    assert_eq!(expected, produced);
}

#[test]
fn test_io_parser() {
    let data = "[1, 2, 3]";
    let reader = ~io::BufReader::new(data.as_bytes());
    let parser = YamlIoParser::init(reader);
    let expected = ~[
        YamlStreamStartEvent(ffi::YamlUtf8Encoding),
        YamlDocumentStartEvent(None, ~[], true),
        YamlSequenceStartEvent(YamlSequenceParam{anchor: None, tag: None, implicit: true, style: ffi::YamlFlowSequenceStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[49u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[50u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[51u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlSequenceEndEvent,
        YamlDocumentEndEvent(true),
        YamlStreamEndEvent
    ];

    let mut produced = ~[];
    let mut stream = parser.parse();

    loop {
        match stream.next_event() {
            Ok(YamlNoEvent) => {
                break;
            },
            Ok(evt) => {
                produced.push(evt);
            },
            Err(err) => {
                fail!("{:?}", err);
            }
        }
    }

    assert_eq!(expected, produced);
}

#[test]
fn test_byte_parser_mapping() {
    let data = "{\"a\": 1, \"b\":2}";
    let parser = YamlByteParser::init(data.as_bytes());
    let expected = ~[
        YamlStreamStartEvent(ffi::YamlUtf8Encoding),
        YamlDocumentStartEvent(None, ~[], true),
        YamlMappingStartEvent(YamlSequenceParam{anchor: None, tag: None, implicit: true, style: ffi::YamlFlowSequenceStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[97u8], plain_implicit: false, quoted_implicit: true, style: ffi::YamlDoubleQuotedScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[49u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[98u8], plain_implicit: false, quoted_implicit: true, style: ffi::YamlDoubleQuotedScalarStyle}),
        YamlScalarEvent(YamlScalarParam{anchor: None, tag: None, value: ~[50u8], plain_implicit: true, quoted_implicit: false, style: ffi::YamlPlainScalarStyle}),
        YamlMappingEndEvent,
        YamlDocumentEndEvent(true),
        YamlStreamEndEvent
    ];

    let mut produced = ~[];
    let mut stream = parser.parse();

    loop {
        match stream.next_event() {
            Ok(YamlNoEvent) => {
                break;
            },
            Ok(evt) => {
                produced.push(evt);
            },
            Err(err) => {
                fail!("{:?}", err);
            }
        }
    }

    assert_eq!(expected, produced);
}

#[test]
fn test_parser_error() {
    let data = "\"ab";
    let parser = YamlByteParser::init(data.as_bytes());
    let mut stream = parser.parse();

    let stream_start = stream.next_event();
    assert_eq!(Ok(YamlStreamStartEvent(ffi::YamlUtf8Encoding)), stream_start);

    let stream_err = stream.next_event();
    match stream_err {
        Ok(evt) => fail!("unexpected result: {:?}", evt),
        Err(err) => assert_eq!(YamlScannerError, err.kind)
    }
}
