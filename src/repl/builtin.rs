
use std::{collections::HashMap, sync::Arc};

use log::{error};
use static_init::dynamic;

use crate::core::Intel8080;

#[derive(Clone, Debug)]
struct Builtin {
    exec: fn(&mut Intel8080, Vec<String>),
    name: &'static str,
    descr: &'static str,
    help: &'static str,
}

#[dynamic]
static BUILTINS: Vec<Arc<Builtin>> = Vec::from([
    Arc::new(Builtin {
        exec: jmp,
        name: "SetPos",
        descr: "Sets the Program Counter position in memory",
        help: "setpos [position::unsigned int]"
    }),
    Arc::new(Builtin {
        exec: help,
        name: "Help",
        descr: "Prints the help message",
        help: "help",
    }),
]);

#[dynamic]
static BUILTIN_MAP: HashMap<String, Arc<Builtin>> = vec![
    (String::from("goto")  , BUILTINS[0].clone()),
    (String::from("setpos"), BUILTINS[0].clone()),
    (String::from("jump")  , BUILTINS[0].clone()),
    (String::from("help")  , BUILTINS[1].clone()),
].iter().cloned().collect();

pub fn run (cpu: &mut Intel8080, line: String) {
    let args: Vec<String> = line.split(" ").map(|s| String::from(s)).collect();

    if valid_command(args[0].clone()) {
        (BUILTIN_MAP.get(&args[0]).unwrap().exec)(cpu, args);
    } else {
        error!("No such command '{}'", args[0]);
    }
}

fn valid_command(cmd: String) -> bool {
    BUILTIN_MAP.contains_key(cmd.as_ref() as &str)
}

fn jmp(cpu: &mut Intel8080, args: Vec<String>) {
    let radix = 
        if args[1].starts_with("0x") || args[1].chars().filter(|&c| c >= 'A' && c <= 'F').count() != 0 {
            16
        } else if args[1].starts_with("0") {
            8
        } else {
            10
        };

    let num = u16::from_str_radix(
        if args[1].starts_with("0x") {
                &args[1].split_at(2).1
            } else {
                &args[1]
            }, radix);

    if let Err(e) = num.clone() {
        error!("Invalid number '{}'. Error: {}", args[1], e);
        return;
    }

    cpu.registers.set_dword("PC", num.unwrap());
}

fn help(_: &mut Intel8080, args: Vec<String>) {
    if args.len() > 1 {
        if !BUILTIN_MAP.contains_key(&args[1]) {
            error!("No such command '{}'", args[1]);
            return;
        }

        println!("{}", BUILTIN_MAP[&args[1]].help)
    } else {
        for command in BUILTINS.iter() {
            println!("{}: {}", command.name, command.descr);
        }
    }
}