use ffi;
use parser::YamlMark;
use std::libc;
use std::c_str::CString;
use std::c_vec::CVec;
use std::ptr;
use std::str;
use std::cast;

pub struct YamlDocument {
    document_mem: ffi::yaml_document_t
}

impl Drop for YamlDocument {
    fn drop(&mut self) {
        unsafe {
            ffi::yaml_document_delete(&mut self.document_mem);
        }
    }
}

pub enum YamlNode<'r> {
    YamlScalarNode(YamlScalarData),
    YamlSequenceNode(YamlSequenceData<'r>),
    YamlMappingNode(YamlMappingData<'r>),
}

pub struct YamlScalarData {
    pub tag: Option<~str>,
    pub value: ~str,
    pub style: ffi::YamlScalarStyle,
    pub start_mark: YamlMark,
    pub end_mark: YamlMark
}

pub struct YamlSequenceData<'r> {
    doc: &'r YamlDocument,
    pub tag: Option<~str>,
    start: *libc::c_int,
    top: *libc::c_int,
    pub style: ffi::YamlSequenceStyle,
    pub start_mark: YamlMark,
    pub end_mark: YamlMark
}

impl<'r> YamlSequenceData<'r> {
    pub fn values(&self) -> YamlSequenceIter<'r> {
        YamlSequenceIter {
            doc: self.doc,
            top: self.top,
            ptr: self.start
        }
    }
}

pub struct YamlSequenceIter<'r> {
    doc: &'r YamlDocument,
    top: *libc::c_int,
    ptr: *libc::c_int
}

impl<'r> Iterator<YamlNode<'r>> for YamlSequenceIter<'r> {
    fn next(&mut self) -> Option<YamlNode<'r>> {
        if self.ptr == self.top {
            None
        } else {
            unsafe {
                let next_node = self.doc.get_node(*self.ptr);

                self.ptr = self.ptr.offset(1);

                Some(next_node)
            }
        }
    }
}

pub struct YamlMappingData<'r> {
    doc: &'r YamlDocument,
    pub tag: Option<~str>,
    start: *ffi::yaml_node_pair_t,
    top: *ffi::yaml_node_pair_t,
    pub style: ffi::YamlSequenceStyle,
    pub start_mark: YamlMark,
    pub end_mark: YamlMark
}

impl<'r> YamlMappingData<'r> {
    pub fn pairs(&self) -> YamlMappingIter<'r> {
        YamlMappingIter {
            doc: self.doc,
            top: self.top,
            ptr: self.start
        }
    }
}

pub struct YamlMappingIter<'r> {
    doc: &'r YamlDocument,
    top: *ffi::yaml_node_pair_t,
    ptr: *ffi::yaml_node_pair_t
}

impl<'r> Iterator<(YamlNode<'r>, YamlNode<'r>)> for YamlMappingIter<'r> {
    fn next(&mut self) -> Option<(YamlNode<'r>, YamlNode<'r>)> {
        if self.ptr == self.top {
            None
        } else {
            unsafe {
                let next_key = self.doc.get_node((*self.ptr).key);
                let next_value = self.doc.get_node((*self.ptr).value);

                self.ptr = self.ptr.offset(1);

                Some((next_key, next_value))
            }
        }
    }
}

impl YamlDocument {
    unsafe fn load<'r>(&'r self, node_ptr: *ffi::yaml_node_t) -> YamlNode<'r> {
        if node_ptr == ptr::null() {
            fail!("empty node")
        }
        let node = &*node_ptr;

        let tag = CString::new(node.tag as *i8, false).as_str().map(|s| s.into_owned());
        let start_mark = YamlMark::conv(&node.start_mark);
        let end_mark = YamlMark::conv(&node.end_mark);
        match node.node_type {
            ffi::YAML_SCALAR_NODE => {
                let scalar_data: &ffi::yaml_scalar_node_t = cast::transmute(&node.data);
                let value = str::from_utf8(CVec::new(scalar_data.value as *mut u8, scalar_data.length as uint).as_slice()).unwrap().into_owned();
                YamlScalarNode(YamlScalarData {
                    tag: tag,
                    value: value,
                    style: scalar_data.style,
                    start_mark: start_mark,
                    end_mark: end_mark
                })
            },
            ffi::YAML_SEQUENCE_NODE => {
                let sequence_data: &ffi::yaml_sequence_node_t = cast::transmute(&node.data);
                let start = sequence_data.items.start as *libc::c_int;
                let top = sequence_data.items.top as *libc::c_int;
                YamlSequenceNode(YamlSequenceData {
                    doc: self,
                    tag: tag,
                    start: start,
                    top: top,
                    style: sequence_data.style,
                    start_mark: start_mark,
                    end_mark: end_mark
                })
            },
            ffi::YAML_MAPPING_NODE => {
                let mapping_data: &ffi::yaml_sequence_node_t = cast::transmute(&node.data);
                let start = mapping_data.items.start as *ffi::yaml_node_pair_t;
                let top = mapping_data.items.top as *ffi::yaml_node_pair_t;
                YamlMappingNode(YamlMappingData {
                    doc: self,
                    tag: tag,
                    start: start,
                    top: top,
                    style: mapping_data.style,
                    start_mark: start_mark,
                    end_mark: end_mark
                })
            },
            _ => fail!("invalid node")
        }
    }

    unsafe fn get_node<'r>(&'r self, index: libc::c_int) -> YamlNode<'r> {
        let node_ptr = ffi::yaml_document_get_node(&self.document_mem, index);
        self.load(node_ptr)
    }

    pub fn root<'r>(&'r self) -> YamlNode<'r> {
        unsafe {
            let node_ptr = ffi::yaml_document_get_root_node(&self.document_mem);
            self.load(node_ptr)
        }
    }
}
