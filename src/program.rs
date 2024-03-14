use std::{fmt, slice::Iter};
use crate::utils::read_bytes;

pub enum InstructionsTypes {
    CP = 0x0,
    LD = 0x1,
    ST = 0x2,
    LI = 0x3,
    LR = 0x4,
    JZ = 0x5,
    AD = 0x8,
    JN = 0x6,
    OP = 0x7,
    SU = 0x9,
    // 0xa is Unused
    MU = 0xb,
    SL = 0xc,
    XR = 0xd,
    SR = 0xe,
    HL = 0xf,
}

impl From<u8> for InstructionsTypes {
    fn from(value: u8) -> Self {
        match value {
            0x0 => InstructionsTypes::CP,
            0x1 => InstructionsTypes::LD,
            0x2 => InstructionsTypes::ST,
            0x3 => InstructionsTypes::LI,
            0x4 => InstructionsTypes::LR,
            0x5 => InstructionsTypes::JZ,
            0x6 => InstructionsTypes::JN,
            0x7 => InstructionsTypes::OP,
            0x8 => InstructionsTypes::AD,
            0x9 => InstructionsTypes::SU,
            0xb => InstructionsTypes::MU,
            0xc => InstructionsTypes::SL,
            0xd => InstructionsTypes::XR,
            0xe => InstructionsTypes::SR,
            0xf => InstructionsTypes::HL,
            _ => panic!("Invalid instruction value"),
        }
    }
}

pub struct Instruction {
    pub instr: InstructionsTypes,
    pub opa: u8,
    pub opb: u8
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instr_dis = match self.instr {
            InstructionsTypes::CP => "CP",
            InstructionsTypes::LD => "LD",
            InstructionsTypes::ST => "ST",
            InstructionsTypes::LI => "LI",
            InstructionsTypes::LR => "LR",
            InstructionsTypes::JZ => "JZ",
            InstructionsTypes::JN => "JN",
            InstructionsTypes::OP => "OP",
            InstructionsTypes::AD => "AD",
            InstructionsTypes::SU => "SU",
            InstructionsTypes::MU => "MU",
            InstructionsTypes::SL => "SL",
            InstructionsTypes::XR => "XR",
            InstructionsTypes::SR => "SR",
            InstructionsTypes::HL => "HL",
        };
        write!(f, "{} {:>3}, {:>3}", instr_dis, self.opa, self.opb)
    }
}

impl Instruction {
    pub fn new(instr: u8, opa: u8, opb: u8) -> Self {
        Instruction {
            instr: instr.into(),
            opa,
            opb
        }
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    data: Vec<u8>
}

impl Program {
    pub fn load(path: &str) -> Self {
        let instruction_data: Vec<(u8, u8)> = read_bytes(path);
        let mut instructions = Vec::new();  // Instructions with operand are on two bytes
        let mut data = Vec::new();  // Static data after the program code.

        // After this ff, data section begins
        let last_ff = instruction_data.len() - instruction_data.iter().rev().position(|&byte| byte == (0xff, 0xff)).unwrap() - 1;

        for (index, instruction) in instruction_data.iter().enumerate() {
            if index > last_ff {
                data.push(instruction.0);
                data.push(instruction.1);
                continue;
            }

            let instr = instruction.0 >> 4;
            let opa = instruction.0 & 0x0f;
            let opb = instruction.1;

            instructions.push(Instruction::new(instr, opa, opb));
        }

        Program {
            instructions,
            data
        }
    }

    pub fn expand(&self) -> () {
        println!("  #| INS  A,   B");
        for (index, instr) in self.instructions.iter().enumerate() {
            println!("{:>3}| {}", index, instr);
        }
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn get_data(&self, index: usize) -> Option<&u8> {
        self.data.get(index)
    }
}


impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program {{ instructions: {}, data: {} byte(s) }}", self.instructions.len(), self.data.len())
    }
}