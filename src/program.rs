use crate::utils::read_bytes;

enum InstructionsTypes {
    CP=0x0,
    LD=0x1,
    ST=0x2,
    LI=0x3,
    LR=0x4,
    JZ=0x5,
    AD=0x8,
    JN=0x6,
    OP=0x7,
    SU=0x9,
    // 0xa is Unused
    MU = 0xb,
    SL = 0xc,
    XR = 0xd,
    SR = 0xe,
    HL = 0xf,
}

struct Instruction {
    instr: InstructionsTypes,
    opa: u8,
    opb: u8
}

pub struct Program {
    instructions: Vec<Instruction>,
    data: Vec<u8>
}

impl Program {
    pub fn load(path: &str) -> Self {
        let instruction_data: Vec<(u8, u8)> = read_bytes(path);
        let instructions = Vec::new();  // Instructions with operand are on two bytes
        let data = Vec::new();  // Static data after the program code.

        let last_ff = instruction_data.len() - instruction_data.iter().rev().position(|&byte| byte == (0xff, 0xff)).unwrap() - 1;
        println!("Last position is {} : {:?}", last_ff, instruction_data[last_ff]);

        Program {
            instructions,
            data
        }
    }
}
