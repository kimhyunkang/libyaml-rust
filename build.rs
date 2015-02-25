#![feature(fs)]
#![feature(io)]
#![feature(process)]
#![feature(env)]
#![feature(path)]

use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path;
use std::env;

fn main()
{
    let dir_var = env::var("OUT_DIR").unwrap();
    let dir = path::Path::new(&dir_var).join("codegen");
    let out_file = dir.to_str().unwrap();
    Command::new("gcc").arg("src/codegen/type_size.c")
                       .arg("-o")
                       .arg(out_file)
                       .status()
                       .unwrap();
    let output = Command::new(out_file).output().unwrap();
    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(output.stderr.as_slice()));
    }
    let mut f = match File::create("src/type_size.rs") {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };
    f.write_all(&output.stdout[..]).unwrap();
}
