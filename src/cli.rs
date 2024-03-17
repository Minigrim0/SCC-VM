use clap::{ArgGroup, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(author="Minigrim0", version, about="An asm 816 interactive (de)compiler")]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["compile", "decompile", "run"]),
))]
pub struct Arguments {
    /// Decompile a binary file into readable `816` code
    #[arg(short, long)]
    pub decompile: bool,

    /// Compile `816` code into a binary
    #[arg(short, long)]
    pub compile: bool,

    /// Run a binary file interactively
    #[arg(short = 'r', long)]
    pub run: bool,

    /// The input file to (de)compile
    #[arg(short, long)]
    pub file: String,

    /// The output file for the (de)compiler output
    #[arg(short, long, default_value_t = String::from("a.out"))]
    pub output: String
}
