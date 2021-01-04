use crate::parser::{ArithmeticCommand, MemorySegment, VMCommand};

#[derive(Default)]
pub struct CodeWriter {
    // TODO
    // curr_filename: String,
    unique_counter: usize,
}

// Move the stack pointer back by one and set D to that value
const POP: &str = "@SP\nAM=M-1\nD=M";
// Make M reference the value behind the current stack pointer value
const LOOK_BACK: &str = "@SP\nA=M-1";

fn comparison(comp: &str, lbl: &str) -> String {
    let commands: Vec<String> = vec![
        POP.into(),
        LOOK_BACK.into(),
        "D=M-D".into(),
        "M=-1".into(),
        format!("@{}", lbl),
        format!("D;{}", comp),
        LOOK_BACK.into(),
        "M=0".into(),
        format!("({})", lbl),
    ];
    commands.join("\n")
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter { unique_counter: 0 }
    }

    // TODO
    pub fn set_file_name(&self) {}

    pub fn write_command(&mut self, command: &VMCommand) -> String {
        match command {
            VMCommand::ArithmeticCommand(arth_cmd) => match arth_cmd {
                ArithmeticCommand::Add => vec![POP, LOOK_BACK, "M=M+D"].join("\n"),
                ArithmeticCommand::Sub => vec![POP, LOOK_BACK, "M=M-D"].join("\n"),
                ArithmeticCommand::Neg => vec![LOOK_BACK, "M=-M"].join("\n"),
                ArithmeticCommand::Eq => {
                    self.unique_counter += 1;
                    let lbl = format!("EQ{}", self.unique_counter);
                    comparison("JEQ", &lbl[..])
                }
                ArithmeticCommand::Gt => {
                    self.unique_counter += 1;
                    let lbl = format!("GT{}", self.unique_counter);
                    comparison("JGT", &lbl[..])
                }
                ArithmeticCommand::Lt => {
                    self.unique_counter += 1;
                    let lbl = format!("LT{}", self.unique_counter);
                    comparison("JLT", &lbl[..])
                }
                ArithmeticCommand::And => vec![POP, LOOK_BACK, "M=M&D"].join("\n"),
                ArithmeticCommand::Or => vec![POP, LOOK_BACK, "M=M|D"].join("\n"),
                ArithmeticCommand::Not => vec![LOOK_BACK, "M=!M"].join("\n"),
            },
            VMCommand::PushCommand(memory_segment, index) => match memory_segment {
                MemorySegment::Constant => vec![
                  format!("@{}", index),
                  "D=A".into(),
                  "@SP".into(),
                  "A=M".into(),
                  "M=D".into(),
                  "@SP".into(),
                  "M=M+1".into(),
                ].join("\n"),
                _ => format!("TODO"),
            },
            VMCommand::PopCommand(_memory_segment, _index) => format!("pop"),
        }
    }
}
