pub mod core;

mod compile;
mod exec;
mod repl;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about, version)]
struct Args {
    ///Hashing algorithm
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    REPL,
    Execute{
        file: String,
    },
    Compile{
        source: String,
    },
}

fn main() {
    let args = Args::parse();

    colog::init();

    match args.mode {
        Mode::REPL => {
            repl::init();
        }
        _ => todo!("compilation and execution not yet supported")
        // Mode::Execute{file} => {
        //     todo!(); //exec::from_binary(file);
        // }
        // Mode::Compile{source} => {
        //     todo!(); //compile::from_asm(source);
        // }
    }
}
