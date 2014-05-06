use ffi;
pub use ffi::{YamlEncoding, YamlScalarStyle, YamlSequenceStyle};
use std::libc::size_t;
use std::cast;
use std::ptr;
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
            ffi::yaml_event_delete(&mut self.event_mem);
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

pub struct YamlEventStream<'r> {
    parser: ~YamlByteParser<'r>,
    end_stream: bool
}

impl<'r> Iterator<YamlEvent> for YamlEventStream<'r> {
    fn next(&mut self) -> Option<YamlEvent> {
        if self.end_stream {
            None
        } else {
            unsafe {
                let evt = self.parser.parse_event();
                match evt {
                    Some(YamlStreamEndEvent) => {
                        self.end_stream = true;
                    },
                    None => {
                        self.end_stream = true;
                    }
                    _ => ()
                }
                evt
            }
        }
    }
}

pub struct YamlByteParser<'r> {
    parser_mem: ffi::yaml_parser_t,
}

impl<'r> YamlByteParser<'r> {
    pub fn init(bytes: &'r [u8]) -> ~YamlByteParser<'r> {
        let mut parser = ~YamlByteParser {
            parser_mem: ffi::new_yaml_parser_t(),
        };

        unsafe {
            let res = ffi::yaml_parser_initialize(&mut parser.parser_mem);
            if res == 0 {
                fail!("failed to initialize yaml_parser_t");
            }
            ffi::yaml_parser_set_input_string(&mut parser.parser_mem, bytes.as_ptr(), bytes.len() as size_t);
        }

        parser
    }

    unsafe fn parse_event(&mut self) -> Option<YamlEvent> {
        let mut event = InternalEvent {
            event_mem: ffi::yaml_event_t::new()
        };

        let res = ffi::yaml_parser_parse(&mut self.parser_mem, &mut event.event_mem);
        if res == 0 {
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

    pub fn parse(~self) -> YamlEventStream<'r> {
        YamlEventStream {
            parser: self,
            end_stream: false
        }
    }
}

impl<'r> Drop for YamlByteParser<'r> {
    fn drop(&mut self) {
        unsafe {
            ffi::yaml_parser_delete(&mut self.parser_mem);
        }
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
    assert_eq!(expected, parser.parse().collect());
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
    assert_eq!(expected, parser.parse().collect());
}
