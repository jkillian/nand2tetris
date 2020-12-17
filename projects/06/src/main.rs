use std::io::{self};

mod code;
mod parser;
mod symbol_table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = parser::parse(&mut io::stdin())?;
    for command in commands.iter() {
        println!("{:016b}", command.to_binary());
    }
    Ok(())
}
