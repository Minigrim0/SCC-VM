use clap::{ArgGroup, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["compile", "decompile"]),
))]
#[clap(group(
    ArgGroup::new("interaction")
        .required(false)
        .args(&["compile", "interactive"]),
))]
pub struct Arguments {
    /// Decompile a binary file into readable `816` code
    #[arg(short, long)]
    pub decompile: bool,

    /// Compile
    #[arg(short, long)]
    pub compile: bool,

    /// Run the binary interactively
    #[arg(short, long)]
    pub interactive: bool,

    /// Number of times to greet
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long, default_value_t = String::from("a.out"))]
    pub output: String
}
