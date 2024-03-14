mod utils;
mod program;
mod machine;

use machine::Machine;
use program::Program;

fn main() {
    let prog = Program::load("examples/binary/demo_xor.bin");
    let mut machine = Machine::new();
    machine.load(prog);
}
