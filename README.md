libyaml-rust
============

[![libyaml-rust on Travis CI][travis-image]][travis]
[![yaml on crates.io][crates-image]][crate]

[travis-image]: https://travis-ci.org/kimhyunkang/libyaml-rust.svg?branch=master
[travis]: https://travis-ci.org/kimhyunkang/libyaml-rust
[crates-image]: http://meritbadge.herokuapp.com/yaml
[crate]: https://crates.io/crates/yaml

[LibYAML][libyaml-home] bindings for [Rust][rust-home]

[libyaml-home]: http://pyyaml.org/wiki/LibYAML
[rust-home]: http://www.rust-lang.org/

Dependencies
------------

* LibYAML 0.1.4 or higher
* Rust 1.1.0 nightly

This crate does not work on Rust 1.0, due to the dependency on [libc](https://github.com/rust-lang/libc)

Usage
-----

Parse from memory

~~~~ {.rust}
extern crate yaml;

use yaml::constructor::*;

yaml::parse_bytes_utf8("[1, 2, 3]".as_bytes()); // => Ok(vec![YamlSequence(~[YamlInteger(1), YamlInteger(2), YamlInteger(3)])])
~~~~

Parse from Reader

~~~~ {.rust}
extern crate yaml;

use std::io::BufReader;
use yaml::constructor::*;

let data = "[1, 2, 3]";
let mut reader = BufReader::new(data.as_bytes());

yaml::parse_io_utf8(&mut reader); // => Ok(vec![YamlSequence(~[YamlInteger(1), YamlInteger(2), YamlInteger(3)])])
~~~~

Todo
----

In the order of what I want to do...

- [x] Emitter functions
- [x] Document iterator
- [x] UTF-16 support
- Complete YAML 1.1 specs
  - [ ] Tag support
  - [ ] [Timestamp type](http://yaml.org/type/timestamp.html)
  - [ ] [Int parser](http://yaml.org/type/int.html)
  - [ ] [Float parser](http://yaml.org/type/float.html)
- [ ] Token functions
