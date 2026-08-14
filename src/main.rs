mod backend;

use std::{fs, process::exit};

use clap::{Parser, Subcommand};
use log::error;

use crate::backend::{compile, exec, repl};

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
    Execute { file: String },
    Compile { source: String },
}

fn main() {
    let args = Args::parse();

    colog::init();

    match args.mode {
        Mode::REPL => {
            repl::init();
        }
        Mode::Compile { source } => {
            match fs::read(source.clone()) {
                Ok(bytes) => {
                    compile::from_source(&source, str::from_utf8(&bytes).unwrap().to_string());
                }
                Err(e) => {
                    error!("Failed to open source file '{source}'\n{e}");
                    exit(1)
                }
            }

            // todo!(); //compile::from_asm(source);
        }
        Mode::Execute{file} => {
            match fs::read(file.clone()) {
                Ok(bytes) => {
                    exec::from_binary(bytes);
                },
                Err(e) => {
                    error!("Failed to open executable file '{file}'\n{e}");
                    exit(1)
                }
            }
        }
    }
}
