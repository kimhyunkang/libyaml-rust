lib: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	mkdir -p build
	rustc src/yaml/lib.rs -g --out-dir=build

build/test: Makefile src/yaml/type_size.rs $(wildcard src/yaml/*.rs)
	mkdir -p build
	rustc --test src/yaml/lib.rs -o $@

check: build/test
	./$<

build/codegen: src/codegen/type_size.c
	mkdir -p build
	gcc $< -o $@

src/yaml/type_size.rs: build/codegen
	$< > $@

clean:
	rm -rf build
