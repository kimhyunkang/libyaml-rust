use ffi;
use ffi::{YamlEncoding, YamlSequenceStyle, YamlScalarStyle};
use ffi::yaml_event_type_t::*;
use std::ffi::{CString, NulError};
use std::mem;
use std::ptr;

use codecs;
use ::error::YamlMark;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct YamlVersionDirective {
    pub major: isize,
    pub minor: isize,
}

#[derive(Debug, PartialEq)]
pub struct YamlTagDirective {
    pub handle: String,
    pub prefix: String,
}

impl YamlTagDirective {
    pub fn to_tag_directive_t(&self) -> Result<ffi::yaml_tag_directive_t, NulError> {
        let handle = try!(CString::new(self.handle.as_bytes()));
        let prefix = try!(CString::new(self.prefix.as_bytes()));
        Ok(ffi::yaml_tag_directive_t {
            handle: handle.as_ptr(),
            prefix: prefix.as_ptr()
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct YamlSequenceParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub implicit: bool,
    pub style: YamlSequenceStyle
}

#[derive(Debug, PartialEq)]
pub struct YamlScalarParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub value: String,
    pub plain_implicit: bool,
    pub quoted_implicit: bool,
    pub style: YamlScalarStyle
}

#[derive(Debug, PartialEq)]
pub enum YamlEventSpec {
    YamlNoEvent,
    YamlStreamStartEvent(YamlEncoding),
    YamlStreamEndEvent,
    YamlDocumentStartEvent(Option<YamlVersionDirective>, Vec<YamlTagDirective>, bool),
    YamlDocumentEndEvent(bool),
    YamlAliasEvent(String),
    YamlScalarEvent(YamlScalarParam),
    YamlSequenceStartEvent(YamlSequenceParam),
    YamlSequenceEndEvent,
    YamlMappingStartEvent(YamlSequenceParam),
    YamlMappingEndEvent,
}

#[derive(Debug)]
pub struct YamlEvent {
    pub spec: YamlEventSpec,
    pub start: YamlMark,
    pub end: YamlMark
}

impl YamlEvent {
    pub unsafe fn load(event: &ffi::yaml_event_t) -> YamlEvent {
        YamlEvent {
            spec: YamlEvent::load_spec(event),
            start: YamlMark::conv(&event.start_mark),
            end: YamlMark::conv(&event.end_mark)
        }
    }

    unsafe fn load_spec(event: &ffi::yaml_event_t) -> YamlEventSpec {
        match event.event_type {
            YAML_NO_EVENT => YamlEventSpec::YamlNoEvent,
            YAML_STREAM_START_EVENT => {
                let evt_data: &ffi::yaml_stream_start_event_t = mem::transmute(&event.data);
                YamlEventSpec::YamlStreamStartEvent(evt_data.encoding)
            },
            YAML_STREAM_END_EVENT => YamlEventSpec::YamlStreamEndEvent,
            YAML_DOCUMENT_START_EVENT => {
                let evt_data: &ffi::yaml_document_start_event_t = mem::transmute(&event.data);
                let vsn_dir = if evt_data.version_directive == ptr::null() {
                    None
                } else {
                    let c_vsn_dir: &ffi::yaml_version_directive_t = mem::transmute(evt_data.version_directive);
                    Some(YamlVersionDirective { major: c_vsn_dir.major as isize, minor: c_vsn_dir.minor as isize })
                };
                let mut tag_dirs = Vec::new();
                let mut tag_ptr = evt_data.tag_directives.start;
                while tag_ptr != ptr::null() && tag_ptr != evt_data.tag_directives.end {
                    let tag_ref: &ffi::yaml_tag_directive_t = mem::transmute(tag_ptr);
                    let handle = codecs::decode_c_str(tag_ref.handle as *const ffi::yaml_char_t).unwrap();
                    let prefix = codecs::decode_c_str(tag_ref.prefix as *const ffi::yaml_char_t).unwrap();
                    tag_dirs.push(YamlTagDirective { handle: handle, prefix: prefix });
                    tag_ptr = tag_ptr.offset(1);
                }
                let implicit = evt_data.implicit != 0;

                YamlEventSpec::YamlDocumentStartEvent(vsn_dir, tag_dirs, implicit)
            },
            YAML_DOCUMENT_END_EVENT => {
                let evt_data: &ffi::yaml_document_end_event_t = mem::transmute(&event.data);
                let implicit = evt_data.implicit != 0;

                YamlEventSpec::YamlDocumentEndEvent(implicit)
            },
            YAML_ALIAS_EVENT => {
                let evt_data: &ffi::yaml_alias_event_t = mem::transmute(&event.data);
                let anchor = codecs::decode_c_str(evt_data.anchor).unwrap();

                YamlEventSpec::YamlAliasEvent(anchor)
            },
            YAML_SCALAR_EVENT => {
                let evt_data: &ffi::yaml_scalar_event_t = mem::transmute(&event.data);
                let value = codecs::decode_buf(evt_data.value, evt_data.length).unwrap();

                YamlEventSpec::YamlScalarEvent(YamlScalarParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    value: value,
                    plain_implicit: evt_data.plain_implicit != 0,
                    quoted_implicit: evt_data.quoted_implicit != 0,
                    style: evt_data.style
                })
            },
            YAML_SEQUENCE_START_EVENT => {
                let evt_data: &ffi::yaml_sequence_start_event_t = mem::transmute(&event.data);

                YamlEventSpec::YamlSequenceStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            YAML_SEQUENCE_END_EVENT => YamlEventSpec::YamlSequenceEndEvent,
            YAML_MAPPING_START_EVENT => {
                let evt_data: &ffi::yaml_mapping_start_event_t = mem::transmute(&event.data);

                YamlEventSpec::YamlMappingStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            YAML_MAPPING_END_EVENT => YamlEventSpec::YamlMappingEndEvent
        }
    }
}
