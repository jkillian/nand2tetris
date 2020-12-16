use crate::parser::{CCommandComp, CCommandDest, CCommandJump, CommandType};

impl CCommandDest {
    fn to_binary(&self) -> u16 {
        match self {
            CCommandDest::None => 0,
            CCommandDest::M => 1,
            CCommandDest::D => 2,
            CCommandDest::DM => 3,
            CCommandDest::A => 4,
            CCommandDest::AM => 5,
            CCommandDest::AD => 6,
            CCommandDest::ADM => 7,
        }
    }
}

impl CCommandComp {
    fn to_binary(&self) -> u16 {
        match self {
            CCommandComp::Zero => 0b0101010,
            CCommandComp::One => 0b0111111,
            CCommandComp::NegOne => 0b0111010,
            CCommandComp::A => 0b0110000,
            CCommandComp::D => 0b0001100,
            CCommandComp::M => 0b1110000,
            CCommandComp::NotA => 0b0110001,
            CCommandComp::NotD => 0b0001101,
            CCommandComp::NotM => 0b1110001,
            CCommandComp::NegA => 0b0110011,
            CCommandComp::NegD => 0b0001111,
            CCommandComp::NegM => 0b1110011,
            CCommandComp::APlusOne => 0b0110111,
            CCommandComp::DPlusOne => 0b0011111,
            CCommandComp::MPlusOne => 0b1110111,
            CCommandComp::AMinusOne => 0b0110010,
            CCommandComp::DMinusOne => 0b0001110,
            CCommandComp::MMinusOne => 0b1110010,
            CCommandComp::DPlusA => 0b0000010,
            CCommandComp::DPlusM => 0b1000010,
            CCommandComp::DMinusA => 0b0010011,
            CCommandComp::DMinusM => 0b1010011,
            CCommandComp::AMinusD => 0b0000111,
            CCommandComp::MMinusD => 0b1000111,
            CCommandComp::DAndA => 0b0000000,
            CCommandComp::DAndM => 0b1000000,
            CCommandComp::DOrA => 0b0010101,
            CCommandComp::DOrM => 0b1010101,
        }
    }
}

impl CCommandJump {
    fn to_binary(&self) -> u16 {
        match self {
            CCommandJump::None => 0,
            CCommandJump::JGT => 1,
            CCommandJump::JEQ => 2,
            CCommandJump::JGE => 3,
            CCommandJump::JLT => 4,
            CCommandJump::JNE => 5,
            CCommandJump::JLE => 6,
            CCommandJump::JMP => 7,
        }
    }
}

impl CommandType {
    pub fn to_binary(&self) -> u16 {
        match self {
            CommandType::ACommand(data) => *data,
            CommandType::CCommand { dest, comp, jump } => {
                (0b111 << 13) | (comp.to_binary() << 6) | (dest.to_binary() << 3) | jump.to_binary()
            }
        }
    }
}
