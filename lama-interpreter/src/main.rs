use std::{error::Error, env, fs, io::Read};

use lama_interpreter::interpreter::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = fs::File::open(&args[1])?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let bytefile = lama_bc::parse(&buffer)?;

    let mut interpreter = Interpreter::new(&bytefile);
    interpreter.run()?;

    Ok(())
}
