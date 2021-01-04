use crate::parser::{ArithmeticCommand, MemorySegment, VMCommand};

#[derive(Default)]
pub struct CodeWriter {
    curr_filename: String,
    unique_counter: usize,
}

// Move the stack pointer back by one and set D to that value
const POP: &str = "@SP\nAM=M-1\nD=M";
// Make M reference the value behind the current stack pointer value
const LOOK_BACK: &str = "@SP\nA=M-1";

const PUSH_D: &str = "@SP
A=M
M=D
@SP
M=M+1";

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

fn memory_segment_pointer_reg(segment: &MemorySegment) -> String {
    match segment {
        MemorySegment::Local => "@LCL",
        MemorySegment::Argument => "@ARG",
        MemorySegment::This => "@THIS",
        MemorySegment::That => "@THAT",
        MemorySegment::Pointer => "@R3",
        MemorySegment::Temp => "@R5",
        _ => panic!("unhandled"),
    }
    .to_string()
}

fn memory_segment_direct_reg(segment: &MemorySegment) -> String {
    match segment {
        MemorySegment::Pointer => "@3",
        MemorySegment::Temp => "@5",
        _ => panic!("unhandled"),
    }
    .to_string()
}

fn memory_segment_pointer_push(segment: &MemorySegment, index: &usize) -> String {
    let commands: Vec<String> = vec![
        memory_segment_pointer_reg(segment),
        "D=M".into(),
        format!("@{}", index),
        "A=A+D".into(),
        "D=M".into(),
        PUSH_D.into(),
    ];
    commands.join("\n")
}

fn memory_segment_direct_push(segment: &MemorySegment, index: &usize) -> String {
    let commands: Vec<String> = vec![
        memory_segment_direct_reg(segment),
        "D=A".into(),
        format!("@{}", index),
        "A=A+D".into(),
        "D=M".into(),
        PUSH_D.into(),
    ];
    commands.join("\n")
}

fn memory_segment_static_push(filename: &str, index: &usize) -> String {
    let commands: Vec<String> = vec![
        format!("@{}.{}", filename, index),
        "D=M".into(),
        PUSH_D.into(),
    ];
    commands.join("\n")
}

fn memory_segment_pointer_pop(segment: &MemorySegment, index: &usize) -> String {
    let commands: Vec<String> = vec![
        memory_segment_pointer_reg(segment),
        "D=M".into(),
        format!("@{}", index),
        "D=A+D".into(),
        "@R13".into(),
        "M=D".into(),
        POP.into(),
        "@R13".into(),
        "A=M".into(),
        "M=D".into(),
    ];
    commands.join("\n")
}

fn memory_segment_direct_pop(segment: &MemorySegment, index: &usize) -> String {
    let commands: Vec<String> = vec![
        memory_segment_direct_reg(segment),
        "D=A".into(),
        format!("@{}", index),
        "D=A+D".into(),
        "@R13".into(),
        "M=D".into(),
        POP.into(),
        "@R13".into(),
        "A=M".into(),
        "M=D".into(),
    ];
    commands.join("\n")
}

fn memory_segment_static_pop(filename: &str, index: &usize) -> String {
    let commands: Vec<String> = vec![POP.into(), format!("@{}.{}", filename, index), "M=D".into()];
    commands.join("\n")
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            unique_counter: 0,
            curr_filename: String::new(),
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.curr_filename = filename.trim_end_matches(".vm").into();
    }

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
                MemorySegment::Constant => {
                    vec![format!("@{}", index), "D=A".into(), PUSH_D.into()].join("\n")
                }
                MemorySegment::Local
                | MemorySegment::Argument
                | MemorySegment::This
                | MemorySegment::That => memory_segment_pointer_push(memory_segment, index),
                MemorySegment::Pointer | MemorySegment::Temp => {
                    memory_segment_direct_push(memory_segment, index)
                }
                MemorySegment::Static => memory_segment_static_push(&self.curr_filename[..], index),
            },
            VMCommand::PopCommand(memory_segment, index) => match memory_segment {
                MemorySegment::Constant => panic!("Cannot pop to contant"),
                MemorySegment::Local
                | MemorySegment::Argument
                | MemorySegment::This
                | MemorySegment::That => memory_segment_pointer_pop(memory_segment, index),
                MemorySegment::Pointer | MemorySegment::Temp => {
                    memory_segment_direct_pop(memory_segment, index)
                }
                MemorySegment::Static => memory_segment_static_pop(&self.curr_filename[..], index),
            },
        }
    }
}
