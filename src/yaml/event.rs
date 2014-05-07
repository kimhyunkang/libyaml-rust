use ffi;
use ffi::{YamlEncoding, YamlSequenceStyle, YamlScalarStyle};
use std::cast;
use std::ptr;
use std::c_str::CString;
use std::c_vec::CVec;

pub struct InternalEvent {
    event_mem: ffi::yaml_event_t
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

fn c_str_into_owned_bytes(c_str: *ffi::yaml_char_t) -> Option<~[u8]> {
    unsafe {
        if c_str == ptr::null() {
            None
        } else {
            Some(CString::new(c_str as *i8, false).as_bytes_no_nul().into_owned())
        }
    }
}

impl YamlEvent {
    pub unsafe fn load(event: &InternalEvent) -> YamlEvent {
        match event.event_mem.event_type {
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
        }
    }
}
