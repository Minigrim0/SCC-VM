mod cli;
mod utils;
mod program;
mod machine;
mod decompiler;

use clap::Parser;

use cli::Arguments;

use decompiler::Decompiler;

fn main() -> Result<(), String> {
    let args = Arguments::parse();

    if args.decompile || args.run {
        let mut dec = Decompiler::new();
        if let Err(e) = dec.load(&args.file) {
            return Err(format!("An error occured while loading the program: {}", e));
        }

        if args.run {
            println!("Running {}", args.file);
            dec.interactive()?;
        } else {
            println!("Decompiling {}", args.file);
           let output = dec.run()?;
           println!("Program output: {}", output);
        }
    }
    if args.compile {
        println!("compiling {} to {}", args.file, args.output);
    }

    Ok(())
}
