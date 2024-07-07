use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod token;
use token::{Token, TokenList};

fn match_reserved(identifier: &str) -> Token {
    match identifier {
        "and" => Token::And,
        "class" => Token::Class,
        "else" => Token::Else,
        "false" => Token::False,
        "for" => Token::For,
        "fun" => Token::Fun,
        "if" => Token::If,
        "nil" => Token::Nil,
        "or" => Token::Or,
        "print" => Token::Print,
        "return" => Token::Return,
        "super" => Token::Super,
        "this" => Token::This,
        "true" => Token::True,
        "var" => Token::Var,
        "while" => Token::While,
        _ => Token::Identifier,
    }
}

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
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let token_list = TokenList::new();

            if !file_contents.is_empty() {
                let result = process_tokens(file_contents, token_list);
                exit(result)
            } else {
                println!("{}", Token::EOF.to_string());
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn truncate_zeros(s: &str) -> String {
    let mut parts: Vec<&str> = s.split('.').collect();
    if parts.len() > 1 {
        parts[1] = parts[1].trim_end_matches('0');
        if parts[1].is_empty() {
            parts[1] = "0";
        }
    }
    parts.join(".")
}

fn process_tokens(file_contents: String, mut token_list: TokenList) -> i32 {
    let mut line_number = 1;
    let mut result = 0;
    let mut chars = file_contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let token = Token::LeftParen;
                println!("{}", &token.to_string());
                token_list.add(token);
            }
            ')' => {
                let token = Token::RightParen;
                println!("{}", &token.to_string());
                token_list.add(token);
            }
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
                Some('/') => loop {
                    let n = chars.peek();
                    if n == Some(&'\n') || n == None {
                        break;
                    } else {
                        chars.next();
                    }
                },
                _ => println!("SLASH / null"),
            },
            '"' => {
                let mut str = String::new();
                loop {
                    match chars.peek() {
                        Some(&'"') => {
                            chars.next();
                            let token = Token::String(str);
                            println!("{}", token.to_string());
                            break;
                        }
                        Some(&c) => {
                            str.push(c);
                            chars.next();
                        }
                        None => {
                            eprintln!("[line {}] Error: Unterminated string.", line_number,);
                            result = 65;
                            break;
                        }
                    }
                }
            }
            ' ' | '\r' | '\t' => {}

            '\n' => line_number += 1,

            c if c.is_digit(10) => {
                let mut number = String::from(c);
                let mut has_decimal = false;

                while let Some(n) = chars.peek() {
                    if n.is_digit(10) {
                        number.push(*n);
                        chars.next();
                    } else if *n == '.' && !has_decimal {
                        has_decimal = true;
                        number.push(*n);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if number.ends_with('.') {
                    let mut display_number = number.clone();
                    display_number.push('0');
                    number.pop();
                    display_number = truncate_zeros(&display_number);
                    println!("NUMBER {} {}", number, display_number);
                    println!("DOT . null");
                } else if !number.contains('.') {
                    let mut display_number = number.clone();
                    display_number.push_str(".0");
                    display_number = truncate_zeros(&display_number);
                    println!("NUMBER {} {}", number, display_number);
                } else {
                    let display_number = truncate_zeros(&number);
                    println!("NUMBER {} {}", number, display_number);
                }
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut identifier = String::from(c);

                while let Some(p) = chars.peek() {
                    if p.is_alphanumeric() || *p == '_' {
                        identifier.push(*p);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let token = match_reserved(&identifier);
                println!("{} {} null", token.to_string(), identifier);
            }

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

    println!("{}", Token::EOF.to_string());

    result
}
