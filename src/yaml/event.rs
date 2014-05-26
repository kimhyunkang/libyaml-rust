use ffi;
use ffi::{YamlEncoding, YamlSequenceStyle, YamlScalarStyle};
use std::mem;
use std::ptr;

use codecs;

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlVersionDirective {
    pub major: int,
    pub minor: int,
}

#[deriving(Eq)]
#[deriving(Show)]
#[deriving(Clone)]
pub struct YamlTagDirective {
    pub handle: String,
    pub prefix: String,
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlSequenceParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub implicit: bool,
    pub style: YamlSequenceStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub struct YamlScalarParam {
    pub anchor: Option<String>,
    pub tag: Option<String>,
    pub value: String,
    pub plain_implicit: bool,
    pub quoted_implicit: bool,
    pub style: YamlScalarStyle
}

#[deriving(Eq)]
#[deriving(Show)]
pub enum YamlEvent {
    YamlNoEvent,
    YamlStreamStartEvent(YamlEncoding),
    YamlStreamEndEvent,
    YamlDocumentStartEvent(Option<YamlVersionDirective>, ~[YamlTagDirective], bool),
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
            ffi::YAML_NO_EVENT => YamlNoEvent,
            ffi::YAML_STREAM_START_EVENT => {
                let evt_data: &ffi::yaml_stream_start_event_t = mem::transmute(&event.data);
                YamlStreamStartEvent(evt_data.encoding)
            },
            ffi::YAML_STREAM_END_EVENT => YamlStreamEndEvent,
            ffi::YAML_DOCUMENT_START_EVENT => {
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
                    let handle = codecs::decode_c_str(tag_ref.handle as *ffi::yaml_char_t).unwrap();
                    let prefix = codecs::decode_c_str(tag_ref.prefix as *ffi::yaml_char_t).unwrap();
                    tag_dirs.push(YamlTagDirective { handle: handle, prefix: prefix });
                    tag_ptr = tag_ptr.offset(1);
                }
                let implicit = evt_data.implicit != 0;

                YamlDocumentStartEvent(vsn_dir, tag_dirs.as_slice().to_owned(), implicit)
            },
            ffi::YAML_DOCUMENT_END_EVENT => {
                let evt_data: &ffi::yaml_document_end_event_t = mem::transmute(&event.data);
                let implicit = evt_data.implicit != 0;

                YamlDocumentEndEvent(implicit)
            },
            ffi::YAML_ALIAS_EVENT => {
                let evt_data: &ffi::yaml_alias_event_t = mem::transmute(&event.data);
                let anchor = codecs::decode_c_str(evt_data.anchor).unwrap();

                YamlAliasEvent(anchor)
            },
            ffi::YAML_SCALAR_EVENT => {
                let evt_data: &ffi::yaml_scalar_event_t = mem::transmute(&event.data);
                let value = codecs::decode_buf(evt_data.value, evt_data.length).unwrap();

                YamlScalarEvent(YamlScalarParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    value: value,
                    plain_implicit: evt_data.plain_implicit != 0,
                    quoted_implicit: evt_data.quoted_implicit != 0,
                    style: evt_data.style
                })
            },
            ffi::YAML_SEQUENCE_START_EVENT => {
                let evt_data: &ffi::yaml_sequence_start_event_t = mem::transmute(&event.data);

                YamlSequenceStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            ffi::YAML_SEQUENCE_END_EVENT => YamlSequenceEndEvent,
            ffi::YAML_MAPPING_START_EVENT => {
                let evt_data: &ffi::yaml_mapping_start_event_t = mem::transmute(&event.data);

                YamlMappingStartEvent(YamlSequenceParam {
                    anchor: codecs::decode_c_str(evt_data.anchor),
                    tag: codecs::decode_c_str(evt_data.tag),
                    implicit: evt_data.implicit != 0,
                    style: evt_data.style
                })
            },
            ffi::YAML_MAPPING_END_EVENT => YamlMappingEndEvent,
            _ => fail!("unknown event type")
        }
    }
}
