use std::fs::File;
use std::io::{stderr, Write};
use std::process::{Command, Output};
use std::path::Path;
use std::env;

fn run_cmd(command: &mut Command) -> Output {
    let stream = stderr();
    let mut f = stream.lock();
    let res = command.output();
    match res {
        Ok(output) => if !output.status.success() {
            write!(&mut f, "Command `{:?}` failed", command).unwrap();
            panic!("{}", String::from_utf8_lossy(&output.stderr[..]))
        } else {
            output
        },
        Err(err) => {
            write!(&mut f, "Command `{:?}` failed", command).unwrap();
            panic!("{}", err)
        }
    }
}

fn main()
{
    let source_file = Path::new("src/codegen/type_size.c");
    if !source_file.exists() {
        panic!("Could not find file: {}", source_file.to_string_lossy());
    }

    let out_dir_var = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => panic!("Could not get env value OUT_DIR: {}", e)
    };
    let out_dir = Path::new(&out_dir_var);
    if !out_dir.exists() {
        panic!("Could not find directory: {}", out_dir.to_string_lossy());
    }

    let out_file = out_dir.join("codegen");

    let mut gcc_cmd = Command::new("gcc");
    gcc_cmd.arg(source_file)
        .arg("-o").arg(&out_file);

    match env::var("LIBYAML_CFLAGS") {
        Ok(compile_flags) => { gcc_cmd.arg(&compile_flags); },
        Err(_) => ()
    }

    run_cmd(&mut gcc_cmd);

    let mut codegen_cmd = Command::new(&out_file);
    let output = run_cmd(&mut codegen_cmd);

    let mut f = match File::create("src/type_size.rs") {
        Ok(f) => f,
        Err(e) => panic!("Could not open file src/type_size.rs: {}", e)
    };

    match f.write_all(&output.stdout[..]) {
        Err(e) => panic!("Could not write to src/type_size.rs: {}", e),
        Ok(_) => ()
    }
}
