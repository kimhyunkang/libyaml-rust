use std::io::{Command, File, Truncate, Write};

fn main()
{
    Command::new("gcc").arg("src/codegen/type_size.c")
                       .arg("-o")
                       .arg("target/codegen")
                       .status()
                       .unwrap();
    let code = Command::new("./target/codegen").output().unwrap();
    if !code.status.success() {
        panic!("{}", String::from_utf8_lossy(code.error.as_slice()));
    }
    let mut f = File::open_mode(&Path::new("src/type_size.rs"), Truncate, Write);
    f.write(code.output.as_slice()).unwrap();
}
