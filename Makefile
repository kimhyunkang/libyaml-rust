lib: Makefile $(wildcard src/yaml/*.rs)
	rustc src/yaml/lib.rs --out-dir=build

test: Makefile $(wildcard src/yaml/*.rs)
	rustc --test src/yaml/lib.rs -o test

check: test
	./test
