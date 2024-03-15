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
        'execution_loop: loop {
            if let Err(e) = self.step() {
                println!("{}", e.to_string());
                break 'execution_loop
            }
            print!("{}", self);
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let program = match &self.program {
            Some(p) => p,
            None => return Err("No program is loaded in memory".to_string())
        };

        let ins = match program.get_instruction(self.pc as usize) {
            Some(i) => i,
            None => return Err(format!("Failed to fecth instruction: pc {}", self.pc))
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
                if (self.reg[ins.opa as usize] & 0b01111111) != 0 {
                    print!("\033[1;0H moving to {}", ins.opb);
                    ins.opb as u32
                } else {
                    self.pc + 1
                }
            },
            InstructionsTypes::OP => {
                if ins.opb == 0 {
                    self.output.push(self.reg[ins.opa as usize] as char);
                } else {
                    for index in ins.opa..(ins.opa + ins.opb) {
                        self.output.push(self.ram[index as usize] as char)
                    }
                }

                self.pc + 1
            },
            InstructionsTypes::AD => {
                self.reg[ins.opa as usize] = ((self.reg[ins.opa as usize] as u16 + self.reg[ins.opb as usize] as u16) % 0xff) as u8;
                self.pc + 1
            },
            InstructionsTypes::SU => {
                match self.reg[ins.opa as usize].checked_sub(self.reg[ins.opb as usize]) {
                    Some(value) => self.reg[ins.opa as usize] = value,
                    None => self.reg[ins.opa as usize] = self.reg[ins.opa as usize] + (255u8 - self.reg[ins.opb as usize])
                };
                self.pc + 1
            },
            InstructionsTypes::MU => {
                self.reg[ins.opa as usize] = (self.reg[ins.opa as usize] as u16 * self.reg[ins.opb as usize] as u16) as u8;
                self.pc + 1
            },
            InstructionsTypes::SL => {
                self.reg[ins.opa as usize] = (self.reg[ins.opa as usize] << self.reg[ins.opa as usize]) & 0xff;
                self.pc + 1
            },
            InstructionsTypes::XR => {
                self.reg[ins.opa as usize] = self.reg[ins.opa as usize] ^ self.reg[ins.opb as usize];
                self.pc + 1
            },
            InstructionsTypes::SR => {
                self.reg[ins.opa as usize] = (self.reg[ins.opa as usize] >> self.reg[ins.opa as usize]) & 0xff;
                self.pc + 1
            },
            InstructionsTypes::HL => {
                panic!("stop");
            },
        };

        Ok(())
    }
}


impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\rMachine {{ program: {:>10}, pc: {:>4}, instr: {:>12}, output {} }}",
            match self.program {Some(_) => "Loaded", None => "Not loaded"},
            self.pc,
            match &self.program {
                Some(p) => match p.get_instruction(self.pc as usize) {
                    Some(ins) => ins.to_string(),
                    None => "--".to_string()
                }
                None => "--".to_string()
            },
            self.output
        )
    }
}