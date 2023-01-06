use std::{env, error::Error, fs, io::Read};

use lama_interpreter::{Environment, Interpreter};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().into_iter().skip(1);
    let file_name = args.next().expect("Expected .bc file path");
    let mut f = fs::File::open(&file_name)?;
    let program_args = args.collect();

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let bytefile = lama_bc::parse(&buffer)?;

    let env: Box<dyn Environment> = if cfg!(feature = "native_env") {
        Box::new(lama_interpreter::NativeEnvironment::new())
    } else {
        Box::new(lama_interpreter::RustEnvironment)
    };

    let mut interpreter = Interpreter::new(&bytefile, env);
    interpreter.run(program_args)?;

    Ok(())
}
