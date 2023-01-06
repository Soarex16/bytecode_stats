use std::{env, error::Error, fs, io::Read};

use lama_interpreter::{Interpreter, RustEnvironment, NativeEnvironment};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().into_iter().skip(1);
    let file_name = args.next().expect("Expected .bc file path");
    let mut f = fs::File::open(&file_name)?;
    let program_args = args.collect();

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let bytefile = lama_bc::parse(&buffer)?;

    let env = if cfg!(all(target_os = "linux", target_arch = "x86")) {
        Box::new(NativeEnvironment)
    } else {
        Box::new(RustEnvironment)
    };

    let mut interpreter = Interpreter::new(&bytefile, env);
    interpreter.run(program_args)?;

    Ok(())
}
