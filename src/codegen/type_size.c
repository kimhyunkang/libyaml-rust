#include <yaml.h>
#include <stdio.h>

int main()
{
    printf("use libc::c_int;\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_parser_mem_t = [c_int; %lu];\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("pub fn new_yaml_parser_mem_t() -> yaml_parser_mem_t {\n");
    printf("    [0; %lu]\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("}\n\n");

    yaml_event_t dummy_event;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_event_data_t = [c_int; %lu];\n", sizeof(dummy_event.data) / sizeof(int));
    printf("pub fn new_yaml_event_data_t() -> yaml_event_data_t {\n");
    printf("    [0; %lu]\n", sizeof(dummy_event.data) / sizeof(int));
    printf("}\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    printf("#[repr(u%lu)]\n", ((size_t)(&dummy_event.data) - (size_t)(&dummy_event)) * 8);
    printf("#[derive(Debug, PartialEq, Copy)]\n");
    printf("pub enum yaml_event_type_t {\n");
    printf("    /** An empty event. */\n");
    printf("    YAML_NO_EVENT = 0,\n\n");

    printf("    /** A STREAM-START event. */\n");
    printf("    YAML_STREAM_START_EVENT,\n");
    printf("    /** A STREAM-END event. */\n");
    printf("    YAML_STREAM_END_EVENT,\n\n");

    printf("    /** A DOCUMENT-START event. */\n");
    printf("    YAML_DOCUMENT_START_EVENT,\n");
    printf("    /** A DOCUMENT-END event. */\n");
    printf("    YAML_DOCUMENT_END_EVENT,\n\n");

    printf("    /** An ALIAS event. */\n");
    printf("    YAML_ALIAS_EVENT,\n");
    printf("    /** A SCALAR event. */\n");
    printf("    YAML_SCALAR_EVENT,\n\n");

    printf("    /** A SEQUENCE-START event. */\n");
    printf("    YAML_SEQUENCE_START_EVENT,\n");
    printf("    /** A SEQUENCE-END event. */\n");
    printf("    YAML_SEQUENCE_END_EVENT,\n\n");

    printf("    /** A MAPPING-START event. */\n");
    printf("    YAML_MAPPING_START_EVENT,\n");
    printf("    /** A MAPPING-END event. */\n");
    printf("    YAML_MAPPING_END_EVENT\n");
    printf("}\n\n");

    yaml_parser_t dummy_parser;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_parser_input_t = [c_int; %lu];\n\n", sizeof(dummy_parser.input) / sizeof(int));
    printf("pub fn new_yaml_parser_input_t() -> yaml_parser_input_t {\n");
    printf("    [0; %lu]\n", sizeof(dummy_parser.input) / sizeof(int));
    printf("}\n\n");

    yaml_emitter_t dummy_emitter;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_emitter_output_t = [c_int; %lu];\n\n", sizeof(dummy_emitter.output) / sizeof(int));
    printf("pub fn new_yaml_emitter_output_t() -> yaml_emitter_output_t {\n");
    printf("    [0; %lu]\n", sizeof(dummy_emitter.output) / sizeof(int));
    printf("}\n\n");

    yaml_node_t dummy_node;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_node_data_t = [c_int; %lu];\n\n", sizeof(dummy_node.data) / sizeof(int));
    printf("pub fn new_yaml_node_data_t() -> yaml_node_data_t {\n");
    printf("    [0; %lu]\n", sizeof(dummy_node.data) / sizeof(int));
    printf("}\n\n");

    printf("#[cfg(test)]\n");
    printf("pub static YAML_PARSER_T_SIZE:usize = %lu;\n", sizeof(yaml_parser_t));
    printf("#[cfg(test)]\n");
    printf("pub static YAML_EMITTER_T_SIZE:usize = %lu;\n", sizeof(yaml_emitter_t));
    printf("#[cfg(test)]\n");
    printf("pub static YAML_EVENT_T_SIZE:usize = %lu;\n", sizeof(yaml_event_t));
    printf("#[cfg(test)]\n");
    printf("pub static YAML_DOCUMENT_T_SIZE:usize = %lu;\n", sizeof(yaml_document_t));
    printf("#[cfg(test)]\n");
    printf("pub static YAML_NODE_T_SIZE:usize = %lu;\n", sizeof(yaml_node_t));

    return 0;
}
