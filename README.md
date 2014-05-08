libyaml-rust
============

[![libyaml-rust on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/kimhyunkang/libyaml-rust.png
[travis]: https://travis-ci.org/kimhyunkang/libyaml-rust

[LibYAML][libyaml-home] bindings for [Rust][rust-home]

[libyaml-home]: http://pyyaml.org/wiki/LibYAML
[rust-home]: http://www.rust-lang.org/

Dependencies
------------

* LibYAML 0.1.4 or higher
* Latest Rust compiler (0.11-pre)

Usage
-----

Parse from memory

~~~~ {.rust}
extern crate yaml;

use yaml::constructor::*;

yaml::parse_bytes("[1, 2, 3]".as_bytes()); // => Ok(vec![YamlSequence(~[YamlInteger(1), YamlInteger(2), YamlInteger(3)])])
~~~~

Parse from Reader

~~~~ {.rust}
extern crate yaml;

use std::io::BufReader;
use yaml::constructor::*;

let data = "[1, 2, 3]";
let reader = ~BufReader::new(data.as_bytes());

yaml::parse_io(reader); // => Ok(vec![YamlSequence(~[YamlInteger(1), YamlInteger(2), YamlInteger(3)])])
~~~~

Todo
----

In the order of what I want to do...

- [ ] Emitter functions
- [ ] Document iterator
- [ ] UTF-16 support
- Complete YAML 1.1 specs
  - [ ] Tag support
  - [ ] Timestamp type
- [ ] Token functions
