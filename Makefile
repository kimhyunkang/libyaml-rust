default: src/type_size.rs

target/codegen: src/codegen/type_size.c
	mkdir -p target
	gcc $< -o $@

src/type_size.rs: target/codegen
	$< > $@

clean:
	rm -rf target
