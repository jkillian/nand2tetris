use std::str::FromStr;

use pest::Parser;
use strum_macros::EnumString;

#[derive(Parser)]
#[grammar = "vm.pest"]
struct VMParser;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MemorySegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug)]
pub enum VMCommand {
    ArithmeticCommand(ArithmeticCommand),
    PushCommand(MemorySegment, usize),
    PopCommand(MemorySegment, usize),
}

pub fn parse_file_contents(source: &str) -> Result<Vec<VMCommand>, Box<dyn std::error::Error>> {
    let pairs = VMParser::parse(Rule::program, source)?;
    let commands: Result<Vec<_>, _> = pairs
        .map(|pair| match pair.as_rule() {
            Rule::c_arithmetic => Ok(VMCommand::ArithmeticCommand(
                ArithmeticCommand::from_str(pair.as_str()).unwrap(),
            )),
            Rule::c_push => {
                let mut pairs = pair.into_inner();
                let memory_segment =
                    MemorySegment::from_str(pairs.next().unwrap().as_str()).unwrap();
                let index = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
                Ok(VMCommand::PushCommand(memory_segment, index))
            }
            Rule::c_pop => {
                let mut pairs = pair.into_inner();
                let memory_segment =
                    MemorySegment::from_str(pairs.next().unwrap().as_str()).unwrap();
                let index = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
                Ok(VMCommand::PopCommand(memory_segment, index))
            }
            _ => Err(format!("Unknown pair {}", pair.as_str())),
        })
        .collect();

    Ok(commands?)
}
