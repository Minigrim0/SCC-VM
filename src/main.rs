mod args;
mod utils;
mod program;
mod machine;
mod decompiler;

use clap::Parser;

use args::Arguments;

use decompiler::Decompiler;

fn main() -> Result<(), String> {
    let args = Arguments::parse();

    if args.decompile {
        println!("Decompiling {} {}", args.file, match args.interactive {true => "interactively", false => ""});
        let mut dec = Decompiler::new();
        if let Err(e) = dec.load(&args.file) {
            return Err(format!("An error occured while loading the program: {}", e));
        }

        if args.interactive {
            dec.interactive()?;
        } else {
           let output = dec.run()?;
           println!("Program output: {}", output);
        }
    }
    if args.compile {
        println!("compiling {} to {}", args.file, args.output);
    }

    Ok(())
}
