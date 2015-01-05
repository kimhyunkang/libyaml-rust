use ffi;
use ffi::{YamlEncoding, YamlSequenceStyle, YamlScalarStyle};
use ffi::yaml_event_type_t::*;
use std::mem;
use std::ptr;

use codecs;

#[derive(Show, PartialEq, Copy)]
pub struct YamlVersionDirective {
    pub major: int,
    pub minor: int,
}

#[derive(Show, PartialEq)]
pub struct YamlTagDirective {
    pub handle: String,
    pub prefix: String,
}

#[derive(Show, PartialEq)]
pub struct YamlSequenceParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub implicit: bool,
    pub style: YamlSequenceStyle
}

#[derive(Show, PartialEq)]
pub struct YamlScalarParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub value: String,
    pub plain_implicit: bool,
    pub quoted_implicit: bool,
    pub style: YamlScalarStyle
}

#[derive(Show, PartialEq)]
pub enum YamlEvent {
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

impl YamlEvent {
    pub unsafe fn load(event: &ffi::yaml_event_t) -> YamlEvent {
        match event.event_type {
            YAML_NO_EVENT => YamlEvent::YamlNoEvent,
            YAML_STREAM_START_EVENT => {
                let evt_data: &ffi::yaml_stream_start_event_t = mem::transmute(&event.data);
                YamlEvent::YamlStreamStartEvent(evt_data.encoding)
            },
            YAML_STREAM_END_EVENT => YamlEvent::YamlStreamEndEvent,
            YAML_DOCUMENT_START_EVENT => {
                let evt_data: &ffi::yaml_document_start_event_t = mem::transmute(&event.data);
                let vsn_dir = if evt_data.version_directive == ptr::null() {
                    None
                } else {
                    let c_vsn_dir: &ffi::yaml_version_directive_t = mem::transmute(evt_data.version_directive);
                    Some(YamlVersionDirective { major: c_vsn_dir.major as int, minor: c_vsn_dir.minor as int })
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

                YamlEvent::YamlDocumentStartEvent(vsn_dir, tag_dirs, implicit)
            },
            YAML_DOCUMENT_END_EVENT => {
                let evt_data: &ffi::yaml_document_end_event_t = mem::transmute(&event.data);
                let implicit = evt_data.implicit != 0;

                YamlEvent::YamlDocumentEndEvent(implicit)
            },
            YAML_ALIAS_EVENT => {
                let evt_data: &ffi::yaml_alias_event_t = mem::transmute(&event.data);
                let anchor = codecs::decode_c_str(evt_data.anchor).unwrap();

                YamlEvent::YamlAliasEvent(anchor)
            },
            YAML_SCALAR_EVENT => {
                let evt_data: &ffi::yaml_scalar_event_t = mem::transmute(&event.data);
                let value = codecs::decode_buf(evt_data.value, evt_data.length).unwrap();

                YamlEvent::YamlScalarEvent(YamlScalarParam {
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

                YamlEvent::YamlSequenceStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            YAML_SEQUENCE_END_EVENT => YamlEvent::YamlSequenceEndEvent,
            YAML_MAPPING_START_EVENT => {
                let evt_data: &ffi::yaml_mapping_start_event_t = mem::transmute(&event.data);

                YamlEvent::YamlMappingStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            YAML_MAPPING_END_EVENT => YamlEvent::YamlMappingEndEvent
        }
    }
}
