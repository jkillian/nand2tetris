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
    LabelCommand(String),
    GotoCommand(String),
    IfGotoCommand(String),
    FunctionCommand(String, usize),
    CallCommand(String, usize),
    ReturnCommand,
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
            Rule::c_label => Ok(VMCommand::LabelCommand(
                pair.into_inner().next().unwrap().as_str().into(),
            )),
            Rule::c_goto => Ok(VMCommand::GotoCommand(
                pair.into_inner().next().unwrap().as_str().into(),
            )),
            Rule::c_if_goto => Ok(VMCommand::IfGotoCommand(
                pair.into_inner().next().unwrap().as_str().into(),
            )),
            Rule::c_function => {
                let mut pairs = pair.into_inner();
                let func_label = String::from(pairs.next().unwrap().as_str());
                let num_args = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
                Ok(VMCommand::FunctionCommand(func_label, num_args))
            }
            Rule::c_call => {
                let mut pairs = pair.into_inner();
                let func_label = String::from(pairs.next().unwrap().as_str());
                let num_args = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
                Ok(VMCommand::CallCommand(func_label, num_args))
            }
            Rule::c_return => Ok(VMCommand::ReturnCommand),
            _ => Err(format!("Unknown pair {}", pair.as_str())),
        })
        .collect();

    Ok(commands?)
}
