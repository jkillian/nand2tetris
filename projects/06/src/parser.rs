use crate::symbol_table::SymbolTable;
use std::fmt;
#[derive(Debug, Clone)]
struct SyntaxError(String);

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.0)
    }
}

impl std::error::Error for SyntaxError {}

#[derive(Debug)]
pub enum Command {
    ACommandNum(u16),
    ACommandSym(String),
    CCommand {
        dest: CCommandDest,
        comp: CCommandComp,
        jump: CCommandJump,
    },
    Label(String),
}

pub enum FinalCommand {
    ACommand(u16),
    CCommand {
        dest: CCommandDest,
        comp: CCommandComp,
        jump: CCommandJump,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum CCommandDest {
    None,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

impl CCommandDest {
    fn from_string(str: &str) -> Result<CCommandDest, SyntaxError> {
        match str {
            "" => Ok(CCommandDest::None),
            "M" => Ok(CCommandDest::M),
            "D" => Ok(CCommandDest::D),
            "DM" | "MD" => Ok(CCommandDest::DM),
            "A" => Ok(CCommandDest::A),
            "AM" | "MA" => Ok(CCommandDest::AM),
            "AD" | "DA" => Ok(CCommandDest::AD),
            "AMD" | "ADM" | "MAD" | "MDA" | "DMA" | "DAM" => Ok(CCommandDest::ADM),
            s => Err(SyntaxError(format!("Invalid destination: {}", s))),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CCommandComp {
    Zero,
    One,
    NegOne,
    A,
    D,
    M,
    NotA,
    NotD,
    NotM,
    NegA,
    NegD,
    NegM,
    APlusOne,
    DPlusOne,
    MPlusOne,
    AMinusOne,
    DMinusOne,
    MMinusOne,
    DPlusA,
    DPlusM,
    DMinusA,
    DMinusM,
    AMinusD,
    MMinusD,
    DAndA,
    DAndM,
    DOrA,
    DOrM,
}

impl CCommandComp {
    fn from_string(str: &str) -> Result<CCommandComp, SyntaxError> {
        match str {
            "0" => Ok(CCommandComp::Zero),
            "1" => Ok(CCommandComp::One),
            "-1" => Ok(CCommandComp::NegOne),
            "A" => Ok(CCommandComp::A),
            "D" => Ok(CCommandComp::D),
            "M" => Ok(CCommandComp::M),
            "!A" => Ok(CCommandComp::NotA),
            "!D" => Ok(CCommandComp::NotD),
            "!M" => Ok(CCommandComp::NotM),
            "-A" => Ok(CCommandComp::NegA),
            "-D" => Ok(CCommandComp::NegD),
            "-M" => Ok(CCommandComp::NegM),
            "A+1" => Ok(CCommandComp::APlusOne),
            "D+1" => Ok(CCommandComp::DPlusOne),
            "M+1" => Ok(CCommandComp::MPlusOne),
            "A-1" => Ok(CCommandComp::AMinusOne),
            "D-1" => Ok(CCommandComp::DMinusOne),
            "M-1" => Ok(CCommandComp::MMinusOne),
            "D+A" | "A+D" => Ok(CCommandComp::DPlusA),
            "D+M" | "M+D" => Ok(CCommandComp::DPlusM),
            "D-A" => Ok(CCommandComp::DMinusA),
            "D-M" => Ok(CCommandComp::DMinusM),
            "A-D" => Ok(CCommandComp::AMinusD),
            "M-D" => Ok(CCommandComp::MMinusD),
            "D&A" | "A&D" => Ok(CCommandComp::DAndA),
            "D&M" | "M&D" => Ok(CCommandComp::DAndM),
            "D|A" | "A|D" => Ok(CCommandComp::DOrA),
            "D|M" | "M|D" => Ok(CCommandComp::DOrM),
            s => Err(SyntaxError(format!("Invalid command: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CCommandJump {
    None,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl CCommandJump {
    fn from_string(str: &str) -> Result<CCommandJump, SyntaxError> {
        match str {
            "" => Ok(CCommandJump::None),
            "JGT" => Ok(CCommandJump::JGT),
            "JEQ" => Ok(CCommandJump::JEQ),
            "JGE" => Ok(CCommandJump::JGE),
            "JLT" => Ok(CCommandJump::JLT),
            "JNE" => Ok(CCommandJump::JNE),
            "JLE" => Ok(CCommandJump::JLE),
            "JMP" => Ok(CCommandJump::JMP),
            s => Err(SyntaxError(format!("Invalid jump: {}", s))),
        }
    }
}

pub fn parse(
    stream: &mut impl std::io::Read,
) -> Result<Vec<FinalCommand>, Box<dyn std::error::Error>> {
    let mut symbol_table = SymbolTable::new();
    let mut data = String::new();
    stream.read_to_string(&mut data)?;

    let maybe_orig_commands: Result<Vec<_>, _> = data
        .lines()
        .filter_map(|line| {
            // remove whitespace
            let stripped_line = line.replace(char::is_whitespace, "");
            // remove any comment
            let code_line = match stripped_line.splitn(2, "//").collect::<Vec<&str>>()[..] {
                [code, _comment] => code,
                [code] => code,
                _ => panic!("Unreachable"),
            };
            match code_line {
                s if s.starts_with("@") => match str::parse::<u16>(&s[1..]) {
                    Ok(val) => Some(Ok(Command::ACommandNum(val))),
                    Err(_) => Some(Ok(Command::ACommandSym(String::from(&s[1..])))),
                },
                s if s.starts_with("(") => Some(Ok(Command::Label(String::from(&s[1..s.len()-1])))),
                s if s.starts_with("/") => None,
                s if s.is_empty() => None,
                s => Some(parse_c_command(String::from(s))),
            }
        })
        .collect();
    let orig_commands = maybe_orig_commands?;

    let mut command_counter = 0;
    for command in &orig_commands {
        if let Command::Label(label) = command {
            symbol_table.insert_label(&label, command_counter);
        } else {
            command_counter += 1;
        }

        if let Command::ACommandSym(sym) = command {
            symbol_table.insert_unknown_symbol(&sym);
        }
    }

    symbol_table.finalize();

    let final_commands: Vec<_> = orig_commands
        .iter()
        .filter_map(|c| match c {
            Command::ACommandNum(val) => Some(FinalCommand::ACommand(*val)),
            Command::ACommandSym(sym) => Some(FinalCommand::ACommand(
                *symbol_table.get_value(sym).unwrap(),
            )),
            Command::CCommand { dest, comp, jump } => Some(FinalCommand::CCommand {
                dest: *dest,
                comp: *comp,
                jump: *jump,
            }),
            Command::Label(_) => None,
        })
        .collect();

    Ok(final_commands)
}

fn parse_c_command(data: String) -> Result<Command, SyntaxError> {
    let (dest, rest) = match data.splitn(2, "=").collect::<Vec<&str>>()[..] {
        [pre, suffix] => (CCommandDest::from_string(pre)?, suffix),
        [all] => (CCommandDest::None, all),
        _ => panic!("Unreachable"),
    };
    let (comp, jump) = match rest.splitn(2, ";").collect::<Vec<&str>>()[..] {
        [comp, jump] => (
            CCommandComp::from_string(comp)?,
            CCommandJump::from_string(jump)?,
        ),
        [all] => (CCommandComp::from_string(all)?, CCommandJump::None),
        _ => panic!("Unreachable"),
    };
    Ok(Command::CCommand { dest, comp, jump })
}
