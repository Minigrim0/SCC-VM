mod utils;
mod program;
mod machine;

use machine::Machine;
use program::Program;

fn main() {
    let prog = Program::load("examples/binary/challenge.bin");
    let mut machine = Machine::new();
    machine.load(prog);
    if let Err(e) = machine.run() {
        panic!("An error occured while running the machine: {}", e.to_string());
    }
}
