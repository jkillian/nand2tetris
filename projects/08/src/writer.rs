use crate::parser::{ArithmeticCommand, MemorySegment, VMCommand};

#[derive(Default)]
pub struct CodeWriter {
    curr_filename: String,
    curr_function: Option<String>,
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

fn push_lbl(lbl: &str) -> String {
    vec![
        format!("@{}", lbl),
        "D=M".into(),
        PUSH_D.into(),
    ].join("\n")
}

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
            curr_function: None,
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.curr_filename = filename.trim_end_matches(".vm").into();
    }

    pub fn write_init_code(&mut self) -> String {
        vec![
            "@256",
            "D=A",
            "@SP",
            "M=D",
            "@LCL",
            "M=D",
            "@ARG",
            "M=D",
            "@THIS",
            "M=D",
            "@THAT",
            "M=D",
            &self.write_command(&VMCommand::CallCommand("Sys.init".into(), 0))[..],
        ].join("\n")
    }

    pub fn write_command(&mut self, command: &VMCommand) -> String {
        match command {
            VMCommand::ArithmeticCommand(arth_cmd) => match arth_cmd {
                ArithmeticCommand::Add => vec![POP, LOOK_BACK, "M=M+D"].join("\n"),
                ArithmeticCommand::Sub => vec![POP, LOOK_BACK, "M=M-D"].join("\n"),
                ArithmeticCommand::Neg => vec![LOOK_BACK, "M=-M"].join("\n"),
                ArithmeticCommand::Eq => {
                    let lbl = self.get_unique_lbl("EQ");
                    comparison("JEQ", &lbl[..])
                }
                ArithmeticCommand::Gt => {
                    let lbl = self.get_unique_lbl("GT");
                    comparison("JGT", &lbl[..])
                }
                ArithmeticCommand::Lt => {
                    let lbl = self.get_unique_lbl("LT");
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
            VMCommand::LabelCommand(label) => {
                format!("({})", self.get_fn_scoped_lbl(label))
            }
            VMCommand::GotoCommand(label) => {
                vec![format!("@{}", self.get_fn_scoped_lbl(label)), "0;JMP".into()].join("\n")
            }
            VMCommand::IfGotoCommand(label) => {
                vec![POP.into(), format!("@{}", self.get_fn_scoped_lbl(label)), "D;JNE".into()].join("\n")
            },
            VMCommand::FunctionCommand(func_name, num_args) => {
                self.curr_function = Some(func_name[..].into());
                let mut instrs = vec![format!("({})", func_name)];
                for _ in 0..*num_args {
                    instrs.push("D=0".into());
                    instrs.push(PUSH_D.into());
                }
                instrs.join("\n")
            }
            VMCommand::CallCommand(func_name, num_args) => {
                let return_lbl = self.get_unique_lbl("RT");
                let args_offset = num_args + 5;
                vec![
                    format!("@{}", return_lbl),
                    "D=A".into(),
                    PUSH_D.into(),
                    push_lbl("LCL"),
                    push_lbl("ARG"),
                    push_lbl("THIS"),
                    push_lbl("THAT"),
                    "@SP".into(),
                    "D=M".into(),
                    format!("@{}", args_offset),
                    "D=D-A".into(),
                    "@ARG".into(),
                    "M=D".into(),
                    "@SP".into(),
                    "D=M".into(),
                    "@LCL".into(),
                    "M=D".into(), 
                    format!("@{}", func_name),
                    "0;JMP".into(),
                    format!("({})", return_lbl),
                ].join("\n")
            },
            VMCommand::ReturnCommand => {
                vec![
                    // store return address in R14, it might be overwritten in the next step
                    "@LCL",
                    "D=M",
                    "@5",
                    "A=D-A",
                    "D=M",
                    "@R14",
                    "M=D",

                    // set return value
                    POP,
                    "@ARG",
                    "A=M",
                    "M=D",

                    // set SP to correct location
                    "@ARG",
                    "D=M+1",
                    "@SP",
                    "M=D",

                    // R13 = LCL
                    "@LCL",
                    "D=M",
                    "@R13",
                    "M=D",

                    // Decrement R13 and 
                    "AM=M-1",
                    "D=M",
                    "@THAT",
                    "M=D",

                    "@R13",
                    "AM=M-1",
                    "D=M",
                    "@THIS",
                    "M=D",

                    "@R13",
                    "AM=M-1",
                    "D=M",
                    "@ARG",
                    "M=D",

                    "@R13",
                    "AM=M-1",
                    "D=M",
                    "@LCL",
                    "M=D",


                    "@R14",
                    "A=M",
                    "0;JMP",
                ].join("\n")
            }
        }
    }

    fn get_fn_scoped_lbl(&self, label: &str) -> String {
        format!("{}${}", self.curr_function.as_deref().unwrap_or(""), label)
    }

    fn get_unique_lbl(&mut self, prefix: &str) -> String {
        self.unique_counter += 1;
        format!("{}{}", prefix, self.unique_counter)
    }
}
