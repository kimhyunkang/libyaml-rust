use std::fs::File;
use std::io::Write;
use std::process::{Command, Output};
use std::path;
use std::env;

fn run_cmd(name: &str, command: &mut Command) -> Output {
    let res = command.output();
    match res {
        Ok(output) => if !output.status.success() {
            panic!("{} failed:\n{}", name, String::from_utf8_lossy(&output.stderr[..]))
        } else {
            output
        },
        Err(err) =>
            panic!("{} failed:\n{}", name, err)
    }
}

fn main()
{
    let dir_var = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => panic!("Could not get env value OUT_DIR: {}", e)
    };

    let out_file = path::Path::new(&dir_var).join("codegen");
    let mut gcc_cmd = Command::new("gcc");
    gcc_cmd.arg("src/codegen/type_size.c").arg("-o").arg(&out_file);

    run_cmd("gcc", &mut gcc_cmd);

    let mut codegen_cmd = Command::new(&out_file);
    let output = run_cmd("codegen", &mut codegen_cmd);

    let mut f = match File::create("src/type_size.rs") {
        Ok(f) => f,
        Err(e) => panic!("Could not open file src/type_size.rs: {}", e)
    };

    match f.write_all(&output.stdout[..]) {
        Err(e) => panic!("Could not write to src/type_size.rs: {}", e),
        Ok(_) => ()
    }
}
