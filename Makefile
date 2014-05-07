lib: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	rustc src/yaml/lib.rs -g --out-dir=build

test: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	rustc --test src/yaml/lib.rs -o test

check: test
	./test

build/codegen: src/codegen/type_size.c
	gcc $< -o $@

src/yaml/type_size.rs: build/codegen
	$< > $@
