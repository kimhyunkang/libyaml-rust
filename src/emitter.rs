use ffi;
use error::YamlError;
use event::{YamlVersionDirective, YamlTagDirective};

use std::str;
use std::slice;
use std::ptr;
use std::mem;
use std::ffi::{c_str_to_bytes, CString};
use std::old_io::IoError;
use libc;

pub struct YamlBaseEmitter {
    emitter_mem: ffi::yaml_emitter_t
}

impl YamlBaseEmitter {
    unsafe fn new() -> YamlBaseEmitter {
        YamlBaseEmitter {
            emitter_mem: mem::uninitialized()
        }
    }
}

impl Drop for YamlBaseEmitter {
    fn drop(&mut self) {
        unsafe {
            ffi::yaml_emitter_delete(&mut self.emitter_mem);
        }
    }
}

pub struct YamlEmitter<'r> {
    base_emitter: YamlBaseEmitter,
    writer: &'r mut (Writer+'r),
    io_error: Option<IoError>,
}

impl<'r> YamlEmitter<'r> {
    pub fn init<'a>(writer: &'a mut Writer) -> Box<YamlEmitter<'a>> {
        unsafe {
            let mut emitter = box YamlEmitter {
                base_emitter: YamlBaseEmitter::new(),
                writer: writer,
                io_error: None
            };

            if ffi::yaml_emitter_initialize(&mut emitter.base_emitter.emitter_mem) == 0 {
                panic!("failed to initialize yaml_emitter_t");
            }

            ffi::yaml_emitter_set_output(&mut emitter.base_emitter.emitter_mem, handle_writer_cb, mem::transmute(&mut *emitter));

            emitter
        }
    }

    fn get_error(&mut self) -> YamlError {
        let emitter_mem = &self.base_emitter.emitter_mem;
        unsafe {
            let c_problem = c_str_to_bytes(&emitter_mem.problem);
            let mut error = YamlError {
                kind: emitter_mem.error,
                problem: str::from_utf8(c_problem).map(|s| s.to_string()).ok(),
                io_error: None,
                context: None
            };

            mem::swap(&mut self.io_error, &mut error.io_error);

            return error;
        }
    }

    pub fn emit_stream<F>(&mut self, encoding: ffi::YamlEncoding, f: F) -> Result<(), YamlError>
        where F: Fn(&mut YamlEmitter) -> Result<(), YamlError>
    {
        try!(self.emit_stream_start_event(encoding));
        try!(f(self));
        try!(self.emit_stream_end_event());
        self.flush()
    }

    fn emit_stream_start_event(&mut self, encoding: ffi::YamlEncoding) -> Result<(), YamlError> {
        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_stream_start_event_initialize(&mut event, encoding) == 0 {
                panic!("yaml_stream_start_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    fn emit_stream_end_event(&mut self) -> Result<(), YamlError> {
        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_stream_end_event_initialize(&mut event) == 0 {
                panic!("yaml_stream_end_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn emit_document<F>(&mut self,
            version_directive: Option<YamlVersionDirective>,
            tag_directives: &[YamlTagDirective],
            implicit: bool,
            f: F) -> Result<(), YamlError> where
        F: Fn(&mut YamlEmitter) -> Result<(), YamlError>
    {
        try!(self.emit_document_start_event(version_directive, tag_directives, implicit));
        try!(f(self));
        self.emit_document_end_event(implicit)
    }

    fn emit_document_start_event(&mut self,
            version_directive: Option<YamlVersionDirective>,
            tag_directives: &[YamlTagDirective],
            implicit: bool)
        -> Result<(), YamlError>
    {
        let mut vsn_dir = ffi::yaml_version_directive_t { major: 0, minor: 0 };
        let c_vsn_dir = match version_directive {
            Some(directive) => {
                vsn_dir.major = directive.major as libc::c_int;
                vsn_dir.minor = directive.minor as libc::c_int;
                &vsn_dir as *const ffi::yaml_version_directive_t
            },
            None => ptr::null()
        };

        let c_strs: Vec<(CString, CString)> = tag_directives.iter().map(|tag| {
            (CString::from_slice(tag.handle.as_bytes()), CString::from_slice(tag.prefix.as_bytes()))
        }).collect();
        let c_tag_dirs: Vec<ffi::yaml_tag_directive_t> = c_strs.iter().map(|tuple| {
            ffi::yaml_tag_directive_t {
                handle: tuple.0.as_ptr(),
                prefix: tuple.1.as_ptr(),
            }
        }).collect();
        let tag_dir_start = c_tag_dirs.as_ptr();
        unsafe {
            let mut event = mem::uninitialized();
            let tag_dir_end = tag_dir_start.offset(c_tag_dirs.len() as isize);
            let c_implicit = if implicit { 1 } else { 0 };

            if ffi::yaml_document_start_event_initialize(&mut event, c_vsn_dir, tag_dir_start, tag_dir_end, c_implicit) == 0 {
                panic!("yaml_document_start_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    fn emit_document_end_event(&mut self, implicit: bool) -> Result<(), YamlError> {
        let c_implicit = if implicit { 1 } else { 0 };
        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_document_end_event_initialize(&mut event, c_implicit) == 0 {
                panic!("yaml_stream_end_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn emit_alias_event(&mut self, anchor: &str) -> Result<(), YamlError> {
        let c_anchor = CString::from_slice(anchor.as_bytes());

        unsafe {
            let mut event = mem::uninitialized();

            let ptr = c_anchor.as_ptr();
            if ffi::yaml_alias_event_initialize(&mut event, ptr as *const ffi::yaml_char_t) != 0 {
                panic!("yaml_stream_end_event_initialize failed!")
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn emit_scalar_event(&mut self, anchor: Option<&str>, tag: Option<&str>,
        value: &str, plain_implicit: bool, quoted_implicit: bool,
        style: ffi::YamlScalarStyle) -> Result<(), YamlError>
    {
        let c_anchor = anchor.map(|s| CString::from_slice(s.as_bytes()));
        let anchor_ptr = match c_anchor {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_tag = tag.map(|s| CString::from_slice(s.as_bytes()));
        let tag_ptr = match c_tag {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_plain_implicit = if plain_implicit { 1 } else { 0 };
        let c_quoted_implicit = if quoted_implicit { 1 } else { 0 };

        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_scalar_event_initialize(&mut event,
                    anchor_ptr as *const ffi::yaml_char_t, tag_ptr as *const ffi::yaml_char_t,
                    value.as_ptr(), value.len() as libc::c_int,
                    c_plain_implicit, c_quoted_implicit,
                    style) == 0
            {
                panic!("yaml_scalar_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn emit_sequence<F>(&mut self, anchor: Option<&str>, tag: Option<&str>, implicit: bool,
            style: ffi::YamlSequenceStyle,
            f: F) -> Result<(), YamlError> where
        F: Fn(&mut YamlEmitter) -> Result<(), YamlError>
    {
        try!(self.emit_sequence_start_event(anchor, tag, implicit, style));
        try!(f(self));
        self.emit_sequence_end_event()
    }

    fn emit_sequence_start_event(&mut self, anchor: Option<&str>, tag: Option<&str>, implicit: bool,
        style: ffi::YamlSequenceStyle) -> Result<(), YamlError>
    {
        let c_anchor = anchor.map(|s| CString::from_slice(s.as_bytes()));
        let anchor_ptr = match c_anchor {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_tag = tag.map(|s| CString::from_slice(s.as_bytes()));
        let tag_ptr = match c_tag {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_implicit = if implicit { 1 } else { 0 };

        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_sequence_start_event_initialize(&mut event,
                    anchor_ptr as *const ffi::yaml_char_t, tag_ptr as *const ffi::yaml_char_t,
                    c_implicit, style) == 0
            {
                panic!("yaml_sequence_start_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    fn emit_sequence_end_event(&mut self) -> Result<(), YamlError> {
        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_sequence_end_event_initialize(&mut event) == 0 {
                panic!("yaml_sequence_end_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn emit_mapping<F>(&mut self, anchor: Option<&str>, tag: Option<&str>, implicit: bool,
            style: ffi::YamlSequenceStyle,
            f: F) -> Result<(), YamlError> where
        F: Fn(&mut YamlEmitter) -> Result<(), YamlError>
    {
        try!(self.emit_mapping_start_event(anchor, tag, implicit, style));
        try!(f(self));
        self.emit_mapping_end_event()
    }

    fn emit_mapping_start_event(&mut self, anchor: Option<&str>, tag: Option<&str>, implicit: bool,
        style: ffi::YamlSequenceStyle) -> Result<(), YamlError>
    {
        let c_anchor = anchor.map(|s| CString::from_slice(s.as_bytes()));
        let anchor_ptr = match c_anchor {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_tag = tag.map(|s| CString::from_slice(s.as_bytes()));
        let tag_ptr = match c_tag {
            Some(s) => s.as_ptr(),
            None => ptr::null()
        };
        let c_implicit = if implicit { 1 } else { 0 };

        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_mapping_start_event_initialize(&mut event,
                    anchor_ptr as *const ffi::yaml_char_t, tag_ptr as *const ffi::yaml_char_t,
                    c_implicit, style) == 0
            {
                panic!("yaml_mapping_start_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    fn emit_mapping_end_event(&mut self) -> Result<(), YamlError> {
        unsafe {
            let mut event = mem::uninitialized();

            if ffi::yaml_mapping_end_event_initialize(&mut event) == 0 {
                panic!("yaml_mapping_end_event_initialize failed!");
            }

            if ffi::yaml_emitter_emit(&mut self.base_emitter.emitter_mem, &mut event) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), YamlError> {
        unsafe {
            if ffi::yaml_emitter_flush(&mut self.base_emitter.emitter_mem) != 0 {
                Ok(())
            } else {
                Err(self.get_error())
            }
        }
    }
}

extern fn handle_writer_cb(data: *mut YamlEmitter, buffer: *const u8, size: libc::size_t) -> libc::c_int {
    unsafe {
        let buf = slice::from_raw_buf(&buffer, size as usize);
        let emitter = &mut *data;
        match emitter.writer.write_all(buf) {
            Ok(()) => 1,
            Err(err) => {
                emitter.io_error = Some(err);
                0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::old_io::MemWriter;
    use emitter::YamlEmitter;
    use ffi::YamlEncoding::YamlUtf8Encoding;
    use ffi::YamlScalarStyle::*;
    use ffi::YamlSequenceStyle::*;

    #[test]
    #[allow(unused_must_use)]
    fn event_emitter_sequence_test() {
        let mut writer = MemWriter::new();
        {
            let mut emitter = YamlEmitter::init(&mut writer);
            emitter.emit_stream(YamlUtf8Encoding, |e| {
                e.emit_document(None, &[], true, |e| {
                    e.emit_sequence(None, None, true, YamlFlowSequenceStyle, |e| {
                        try!(e.emit_scalar_event(None, None, "1", true, false, YamlPlainScalarStyle));
                        e.emit_scalar_event(None, None, "2", true, false, YamlPlainScalarStyle)
                    })
                })
            });
            emitter.flush();
        }
        assert_eq!(writer.get_ref(), "[1, 2]\n".as_bytes());
    }

    #[test]
    #[allow(unused_must_use)]
    fn event_emitter_mapping_test() {
        let mut writer = MemWriter::new();
        {
            let mut emitter = YamlEmitter::init(&mut writer);
            emitter.emit_stream(YamlUtf8Encoding, |e| {
                e.emit_document(None, &[], true, |e| {
                    e.emit_mapping(None, None, true, YamlFlowSequenceStyle, |e| {
                        try!(e.emit_scalar_event(None, None, "a", true, false, YamlPlainScalarStyle));
                        try!(e.emit_scalar_event(None, None, "1", true, false, YamlPlainScalarStyle));
                        try!(e.emit_scalar_event(None, None, "b", true, false, YamlPlainScalarStyle));
                        e.emit_scalar_event(None, None, "2", true, false, YamlPlainScalarStyle)
                    })
                })
            });
            emitter.flush();
        }
        assert_eq!(writer.get_ref(), "{a: 1, b: 2}\n".as_bytes());
    }
}
