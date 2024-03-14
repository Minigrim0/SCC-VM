mod utils;
mod program;

use program::Program;

fn main() {
    let prog = Program::load("examples/binary/demo_xor.bin");
    println!("Loaded {}", prog);
    prog.expand();
}
