all: build/.lib.dummy

build/.lib.dummy: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	mkdir -p build
	rustc src/yaml/lib.rs -g --out-dir=build && touch build/.lib.dummy

build/test: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	mkdir -p build
	rustc --test src/yaml/lib.rs -o $@

build/yaml_spec_test: build/.lib.dummy test/yaml_spec_test.rs
	mkdir -p build
	rustc --test test/yaml_spec_test.rs -L build -o build/yaml_spec_test

check: build/test build/yaml_spec_test $(wildcard test/source/*.yml)
	./build/test && ./build/yaml_spec_test

build/codegen: src/codegen/type_size.c
	mkdir -p build
	gcc $< -o $@

src/yaml/type_size.rs: build/codegen
	$< > $@

clean:
	rm -rf build
