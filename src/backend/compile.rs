use log::error;

enum Statement {
    Comment,
    Instruction{
        name: String,
        tokens: Vec<Statement>,
    },
    Ident(String),
    Number(i32),
    None,
}

pub fn from_asm(asm: String) {
    let program: Vec<Statement> = Vec::new();
    let mut line = 1;

    let mut iter = asm.chars().peekable();
    let line = 0;
}

pub fn from_line(line: String, file: String, line_no: usize) -> Result<Statement, String> {
    let mut iter = line.chars().peekable();
    let mut buffer = Statement::None;

    while let Some(ch) = iter.next() {
        match ch {
            ';' => {
                return Ok(Statement::Comment);
            }

            'A'..'Z' | 'a'..'z' => {
                let mut ident = String::new();
                
                while let Some(&ch) = iter.peek() {
                    if !ch.is_alphanumeric() {
                        break;
                    }

                    ident.push(ch);
                    _ = iter.next();
                }

                if let Statement::Instruction { name, tokens } = &mut buffer {
                    tokens.push(Statement::Ident(ident))
                } else {
                    buffer = Statement::Instruction { name: ident, tokens: Vec::new() }
                }
            }

            '0'..'9' => {
                let mut number = String::new();
                
                while let Some(&ch) = iter.peek() {
                    if !ch.is_numeric() {
                        break;
                    }

                    number.push(ch);
                    _ = iter.next();
                }   

                if let Statement::Instruction { name, tokens } = &mut buffer {
                    let radix = 
                        if number.ends_with("H") || number.ends_with("h") {
                            16
                        } else {
                            10
                        };

                    let num = i32::from_str_radix(&number, radix);

                    if let Err(e) = num {
                        return Err(format!("Invalid number '{number}'. {}", e));
                    }

                    tokens.push(Statement::Number(num.unwrap()))
                } else {
                    return Err(format!("Stray number '{number}' in file"));
                }

            }

            _ => todo!("Unfinished")
        }
    }

    Ok(buffer)
}