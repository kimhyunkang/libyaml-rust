#![feature(core)]
#![feature(io)]
#![feature(os)]
#![feature(path)]

use std::io::{Command, File, Truncate, Write};
use std::os;

fn main()
{
    let dir = Path::new(os::getenv("OUT_DIR").unwrap());
    let out_file = dir.join("codegen");
    Command::new("gcc").arg("src/codegen/type_size.c")
                       .arg("-o")
                       .arg(out_file.as_vec())
                       .status()
                       .unwrap();
    let code = Command::new(out_file.as_vec()).output().unwrap();
    if !code.status.success() {
        panic!("{}", String::from_utf8_lossy(code.error.as_slice()));
    }
    let mut f = File::open_mode(&Path::new("src/type_size.rs"), Truncate, Write);
    f.write(code.output.as_slice()).unwrap();
}
