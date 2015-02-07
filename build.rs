#![feature(core)]
#![feature(io)]
#![feature(env)]
#![feature(path)]

use std::old_io::{Command, File, Truncate, Write};
use std::path;
use std::env;

fn main()
{
    let dir_var = env::var("OUT_DIR").unwrap();
    let dir = path::Path::new(&dir_var);
    let out_file = dir.join("codegen");
    Command::new("gcc").arg("src/codegen/type_size.c")
                       .arg("-o")
                       .arg(out_file.to_str().unwrap())
                       .status()
                       .unwrap();
    let code = Command::new(out_file.to_str().unwrap()).output().unwrap();
    if !code.status.success() {
        panic!("{}", String::from_utf8_lossy(code.error.as_slice()));
    }
    let mut f = File::open_mode(&Path::new("src/type_size.rs"), Truncate, Write);
    f.write_all(code.output.as_slice()).unwrap();
}
