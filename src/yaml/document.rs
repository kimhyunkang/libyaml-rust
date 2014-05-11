use libc;

use codecs;
use ffi;
use parser::YamlMark;

use std::ptr;
use std::cast;

pub struct YamlDocument {
    document_mem: ffi::yaml_document_t
}

impl YamlDocument {
    pub unsafe fn parser_load(parser: &mut ffi::yaml_parser_t) -> Option<~YamlDocument> {
        let mut document = ~YamlDocument {
            document_mem: ffi::yaml_document_t::new()
        };

        if ffi::yaml_parser_load(parser, &mut document.document_mem) == 0 {
            None
        } else {
            Some(document)
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::yaml_document_get_root_node(&self.document_mem) == ptr::null()
        }
    }

    unsafe fn load<'r>(&'r self, node_ptr: *ffi::yaml_node_t) -> YamlNode<'r> {
        if node_ptr == ptr::null() {
            fail!("empty node")
        }
        let node = &*node_ptr;
        match node.node_type {
            ffi::YAML_SCALAR_NODE => {
                let scalar_data: &ffi::yaml_scalar_node_t = cast::transmute(&node.data);
                YamlScalarNode(YamlScalarData {
                    doc: self,
                    node: node,
                    data: scalar_data
                })
            },
            ffi::YAML_SEQUENCE_NODE => {
                let sequence_data: &ffi::yaml_sequence_node_t = cast::transmute(&node.data);
                YamlSequenceNode(YamlSequenceData {
                    doc: self,
                    node: node,
                    data: sequence_data
                })
            },
            ffi::YAML_MAPPING_NODE => {
                let mapping_data: &ffi::yaml_sequence_node_t = cast::transmute(&node.data);
                YamlMappingNode(YamlMappingData {
                    doc: self,
                    node: node,
                    data: mapping_data
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

impl Drop for YamlDocument {
    fn drop(&mut self) {
        unsafe {
            ffi::yaml_document_delete(&mut self.document_mem);
        }
    }
}

pub enum YamlNode<'r> {
    YamlScalarNode(YamlScalarData<'r>),
    YamlSequenceNode(YamlSequenceData<'r>),
    YamlMappingNode(YamlMappingData<'r>),
}

pub trait YamlNodeData {
    unsafe fn internal_node<'r>(&'r self) -> &'r ffi::yaml_node_t;

    fn tag(&self) -> Option<~str> {
        unsafe {
            codecs::decode_c_str(self.internal_node().tag)
        }
    }

    fn start_mark(&self) -> YamlMark {
        unsafe {
            YamlMark::conv(&self.internal_node().start_mark)
        }
    }

    fn end_mark(&self) -> YamlMark {
        unsafe {
            YamlMark::conv(&self.internal_node().end_mark)
        }
    }
}

pub struct YamlScalarData<'r> {
    doc: &'r YamlDocument,
    node: &'r ffi::yaml_node_t,
    data: &'r ffi::yaml_scalar_node_t
}

impl<'r> YamlNodeData for YamlScalarData<'r> {
    unsafe fn internal_node<'r>(&'r self) -> &'r ffi::yaml_node_t {
        self.node
    }
}

impl<'r> YamlScalarData<'r> {
    pub fn get_value(&self) -> ~str {
        codecs::decode_buf(self.data.value, self.data.length).unwrap()
    }

    pub fn style(&self) -> ffi::YamlScalarStyle {
        self.data.style
    }
}

pub struct YamlSequenceData<'r> {
    doc: &'r YamlDocument,
    node: &'r ffi::yaml_node_t,
    data: &'r ffi::yaml_sequence_node_t
}

impl<'r> YamlNodeData for YamlSequenceData<'r> {
    unsafe fn internal_node<'r>(&'r self) -> &'r ffi::yaml_node_t {
        self.node
    }
}

impl<'r> YamlSequenceData<'r> {
    pub fn values(&self) -> YamlSequenceIter<'r> {
        YamlSequenceIter {
            doc: self.doc,
            top: self.data.items.top as *libc::c_int,
            ptr: self.data.items.start as *libc::c_int
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
    node: &'r ffi::yaml_node_t,
    data: &'r ffi::yaml_sequence_node_t
}

impl<'r> YamlNodeData for YamlMappingData<'r> {
    unsafe fn internal_node<'r>(&'r self) -> &'r ffi::yaml_node_t {
        self.node
    }
}

impl<'r> YamlMappingData<'r> {
    pub fn pairs(&self) -> YamlMappingIter<'r> {
        YamlMappingIter {
            doc: self.doc,
            top: self.data.items.top as *ffi::yaml_node_pair_t,
            ptr: self.data.items.start as *ffi::yaml_node_pair_t
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

