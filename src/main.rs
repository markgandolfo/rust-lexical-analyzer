use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let result = process_tokens(file_contents);
                exit(result)
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn process_tokens(file_contents: String) -> i32 {
    let mut line_number = 1;
    let mut result = 0;
    let mut chars = file_contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            '.' => println!("DOT . null"),
            ',' => println!("COMMA , null"),
            ';' => println!("SEMICOLON ; null"),
            '=' => match chars.peek() {
                Some('=') => {
                    println!("EQUAL_EQUAL == null");
                    chars.next();
                }
                _ => println!("EQUAL = null"),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    println!("BANG_EQUAL != null");
                    chars.next();
                }
                _ => println!("BANG ! null"),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    println!("GREATER_EQUAL >= null");
                    chars.next();
                }
                _ => println!("GREATER > null"),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    println!("LESS_EQUAL <= null");
                    chars.next();
                }
                _ => println!("LESS < null"),
            },
            '/' => match chars.peek() {
                Some('/') => {
                    loop {
                        let n = chars.peek();
                        if n == Some(&'\n') || n == None {
                            break;
                        } else {
                            chars.next();
                        }
                    }

                    line_number += 1;
                }
                _ => println!("SLASH / null"),
            },

            '\n' => line_number += 1,
            invalid => {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    line_number,
                    String::from(invalid)
                );
                result = 65;
            }
        };
    }

    println!("EOF  null");

    result
}
