use std::fmt;
use crate::program::{InstructionsTypes, Program};

pub struct Machine {
    program: Option<Program>,
    reg: [u8; 16],
    ram: [u8; 256],
    output: String,
    pc: u32
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            program: None,
            reg: [0; 16],
            ram: [0; 256],
            output: String::new(),
            pc: 0
        }
    }

    pub fn load(&mut self, program: Program) -> () {
        self.program = Some(program);
    }

    pub fn run(&mut self) -> Result<(), String> {
        let program = match &self.program {
            Some(p) => p,
            None => return Err("No program is loaded in memory".to_string())
        };

        'execution_loop: loop {
            let ins = match program.get_instruction(self.pc as usize) {
                Some(i) => i,
                None => break 'execution_loop
            };

            self.pc = match ins.instr {
                InstructionsTypes::CP => {
                    for offset in 1..ins.opb {
                        self.ram[offset as usize] = match program.get_data(offset as usize) {
                            Some(v) => *v,
                            None => {
                                println!("Unable to read data from program at index {}, defaulting to 0x0", offset);
                                0x0
                            }
                        };
                    }
                    self.pc + 1
                },
                InstructionsTypes::LD => {
                    self.reg[ins.opa as usize] = self.ram[self.reg[ins.opb as usize] as usize];
                    self.pc + 1
                },
                InstructionsTypes::ST => {
                    self.ram[self.reg[ins.opa as usize] as usize] = self.reg[ins.opb as usize];
                    self.pc + 1
                },
                InstructionsTypes::LI => {
                    self.reg[ins.opa as usize] = ins.opb;
                    self.pc + 1
                },
                InstructionsTypes::LR => {
                    self.reg[ins.opa as usize] = self.reg[ins.opb as usize];
                    self.pc + 1
                },
                InstructionsTypes::JZ => {
                    if self.reg[ins.opa as usize] == 0b10000001 {
                        ins.opb as u32
                    } else {
                        self.pc + 1
                    }
                },
                InstructionsTypes::JN => {
                    self.pc + 1
                },
                InstructionsTypes::OP => {
                    self.pc + 1
                },
                InstructionsTypes::AD => {
                    self.pc + 1
                },
                InstructionsTypes::SU => {
                    self.pc + 1
                },
                InstructionsTypes::MU => {
                    self.pc + 1
                },
                InstructionsTypes::SL => {
                    self.pc + 1
                },
                InstructionsTypes::XR => {
                    self.pc + 1
                },
                InstructionsTypes::SR => {
                    self.pc + 1
                },
                InstructionsTypes::HL => {
                    self.pc + 1
                },
            };
        }
        Ok(())
    }
}


impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine {{ program: {:>10}, pc: {:>4} }}",
            match self.program {Some(_) => "Loaded", None => "Not loaded"},
            self.pc
        )
    }
}