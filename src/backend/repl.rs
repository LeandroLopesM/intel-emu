mod builtin;

use log::error;
use rustyline::{DefaultEditor, error::ReadlineError};

use crate::backend::core::Intel8080;

pub fn init() {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new().unwrap();

    let mut cpu = Intel8080::new(Some(u16::MAX as usize));

    loop {
        match rl.readline(&format!("{:0<4X} > ", cpu.registers.get_dword("PC").unwrap())) {
            Ok(line) => exec_line(&mut cpu, line),
            
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => error!("Failed to read line ({})", err),
        }
    }
}

fn exec_line(cpu: &mut Intel8080, mut line: String) {
    if line.starts_with("/") {
        line.remove(0);
        builtin::run(cpu, line);
        return;
    }
}