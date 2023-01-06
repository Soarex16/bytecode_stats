use std::{env, error::Error, fs, io::Read};

use lama_interpreter::{Interpreter, RustEnvironment};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().into_iter().skip(1);
    let file_name = args.next().expect("Expected .bc file path");
    let mut f = fs::File::open(&file_name)?;
    let program_args = args.collect();

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let bytefile = lama_bc::parse(&buffer)?;

    let mut interpreter = Interpreter::new(&bytefile, Box::new(RustEnvironment));
    interpreter.run(program_args)?;

    Ok(())
}
