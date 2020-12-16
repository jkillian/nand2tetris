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
pub enum CommandType {
    ACommand(u16),
    CCommand {
        dest: CCommandDest,
        comp: CCommandComp,
        jump: CCommandJump,
    },
}

#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(Debug)]
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
) -> Result<Vec<CommandType>, Box<dyn std::error::Error>> {
    let mut data = String::new();
    stream.read_to_string(&mut data)?;

    let result: Result<Vec<_>, _> = data
        .lines()
        .filter_map(|line| {
            let stripped_line = line.replace(char::is_whitespace, "");
            match stripped_line {
                s if s.starts_with("@") => match str::parse::<u16>(&s[1..]) {
                    Ok(val) => Some(Ok(CommandType::ACommand(val))),
                    Err(_) => Some(Err(SyntaxError(format!("Could not parse {}", s)))),
                },
                s if s.starts_with("(") => None,
                s if s.starts_with("/") => None,
                s if s.is_empty() => None,
                s => Some(parse_c_command(s)),
            }
        })
        .collect();

    Ok(result?)
}

fn parse_c_command(data: String) -> Result<CommandType, SyntaxError> {
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
    Ok(CommandType::CCommand { dest, comp, jump })
}
