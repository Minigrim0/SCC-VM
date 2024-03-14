mod utils;
mod program;

use program::Program;

fn main() {
    let prog = Program::load("examples/binary/hello.bin");
}
