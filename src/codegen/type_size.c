#include <yaml.h>
#include <stdio.h>

int main()
{
    printf("use std::libc::c_int;\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_parser_mem_t = [c_int, ..%lu];\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("pub fn new_yaml_parser_mem_t() -> yaml_parser_mem_t {\n");
    printf("    [0, ..%lu]\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("}\n\n");

    yaml_event_t dummy_event;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub type yaml_event_data_t = [c_int, ..%lu];\n", sizeof(dummy_event.data) / sizeof(int));
    printf("pub fn new_yaml_event_data_t() -> yaml_event_data_t {\n");
    printf("    [0, ..%lu]\n", sizeof(dummy_event.data) / sizeof(int));
    printf("}\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    // FIXME: might not work on big endian machines
    printf("pub type yaml_event_type_t = u%lu;\n\n", ((size_t)(&dummy_event.data) - (size_t)(&dummy_event)) * 8);

    yaml_parser_t dummy_parser;

    printf("#[allow(non_camel_case_types)]\n");
    // FIXME: might not work on big endian machines
    printf("pub type yaml_error_type_t = u%lu;\n\n", ((size_t)(&dummy_parser.problem) - (size_t)(&dummy_parser)) * 8);

    printf("#[allow(non_camel_case_types)]\n");
    // FIXME: might not work on big endian machines
    printf("pub type yaml_parser_input_t = [c_int, ..%lu];\n\n", sizeof(dummy_parser.input) / sizeof(int));
    printf("pub fn new_yaml_parser_input_t() -> yaml_parser_input_t {\n");
    printf("    [0, ..%lu]\n", sizeof(dummy_parser.input) / sizeof(int));
    printf("}\n\n");

    printf("#[cfg(test)]\n");
    printf("pub static yaml_parser_t_size:uint = %lu;\n", sizeof(yaml_parser_t));
    printf("#[cfg(test)]\n");
    printf("pub static yaml_event_t_size:uint = %lu;\n", sizeof(yaml_event_t));

    return 0;
}
