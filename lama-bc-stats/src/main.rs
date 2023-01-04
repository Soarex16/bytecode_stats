use lama_bc::{
    self,
    bytecode::{ByteFile, OpCode},
};
use prettytable::{row, Table};
use std::{collections::HashMap, env, error::Error, fs, io::Read};

fn count_stats<'a>(bf: &'a ByteFile) -> HashMap<&'a OpCode, i64> {
    let mut stats = HashMap::<&OpCode, i64>::new();
    for opcode in bf.code.iter() {
        match opcode {
            OpCode::STI
            | OpCode::STA
            | OpCode::ELEM
            | OpCode::END
            | OpCode::RET
            | OpCode::DROP
            | OpCode::DUP
            | OpCode::SWAP => (), // skip opcodes without params
            _ => *stats.entry(opcode).or_insert(0) += 1,
        }
    }
    stats
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = fs::File::open(&args[1])?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let bytefile = lama_bc::parse(&buffer)?;

    let stats = count_stats(&bytefile);
    let mut stats = stats.iter().collect::<Vec<_>>();
    stats.sort_by_key(|&(_, &k)| -k);

    let mut table = Table::new();

    table.add_row(row!["opcode", "freq"]);
    for (k, v) in stats.iter() {
        table.add_row(row![k, v]);
    }
    table.printstd();

    Ok(())
}
