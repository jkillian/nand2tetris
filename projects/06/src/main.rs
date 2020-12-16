use std::io::{self};

mod code;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = parser::parse(&mut io::stdin())?;
    for command in commands.iter() {
        // println!("{:#?}", result.unwrap());
        println!("{:016b}", command.to_binary());
    }
    Ok(())
}
