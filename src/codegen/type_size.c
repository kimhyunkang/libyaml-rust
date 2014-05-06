#include <yaml.h>
#include <stdio.h>

int main()
{
    printf("use std::libc::c_int;\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub struct yaml_parser_t { opaque: [c_int, ..%lu] }\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("pub fn new_yaml_parser_t() -> yaml_parser_t {\n");
    printf("    yaml_parser_t { opaque: [0, ..%lu] }\n", sizeof(yaml_parser_t) / sizeof(int));
    printf("}\n\n");

    yaml_event_t dummy_event;

    printf("#[allow(non_camel_case_types)]\n");
    printf("pub struct yaml_event_data_t { opaque: [c_int, ..%lu] }\n", sizeof(dummy_event.data) / sizeof(int));
    printf("pub fn new_yaml_event_data_t() -> yaml_event_data_t {\n");
    printf("    yaml_event_data_t { opaque: [0, ..%lu] }\n", sizeof(dummy_event.data) / sizeof(int));
    printf("}\n\n");

    printf("#[allow(non_camel_case_types)]\n");
    // FIXME: might not work on big endian machines
    printf("pub type yaml_event_type_t = u%lu;\n\n", ((size_t)(&dummy_event.data) - (size_t)(&dummy_event)) * 8);

    printf("#[cfg(test)]\n");
    printf("pub static yaml_parser_t_size:uint = %lu;\n", sizeof(yaml_parser_t));
    printf("#[cfg(test)]\n");
    printf("pub static yaml_event_t_size:uint = %lu;\n", sizeof(yaml_event_t));

    return 0;
}
