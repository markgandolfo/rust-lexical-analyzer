use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod token;
use token::{Token, TokenList};

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

            let mut token_list = TokenList::new();

            if !file_contents.is_empty() {
                let result = process_tokens(file_contents, &mut token_list);
                token_list.print_all();
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

fn process_tokens(file_contents: String, token_list: &mut TokenList) -> i32 {
    let mut line_number = 1;
    let mut result = 0;
    let mut chars = file_contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => token_list.add(Token::LeftParen),
            ')' => token_list.add(Token::RightParen),
            '{' => token_list.add(Token::LeftBrace),
            '}' => token_list.add(Token::RightBrace),
            '*' => token_list.add(Token::Star),
            '+' => token_list.add(Token::Plus),
            '-' => token_list.add(Token::Minus),
            '.' => token_list.add(Token::Dot),
            ',' => token_list.add(Token::Comma),
            ';' => token_list.add(Token::SemiColon),
            '=' => match chars.peek() {
                Some('=') => {
                    token_list.add(Token::EqualEqual);
                    chars.next();
                }
                _ => token_list.add(Token::Equal),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    token_list.add(Token::BangEqual);
                    chars.next();
                }
                _ => token_list.add(Token::Bang),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    token_list.add(Token::GreaterEqual);
                    chars.next();
                }
                _ => token_list.add(Token::Greater),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    token_list.add(Token::LessEqual);
                    chars.next();
                }
                _ => token_list.add(Token::Less),
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
                _ => token_list.add(Token::Slash),
            },
            '"' => {
                let mut str = String::new();
                loop {
                    match chars.peek() {
                        Some(&'"') => {
                            chars.next();
                            let token = Token::String(str);
                            token_list.add(token);
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

                token_list.add(Token::Number(number.clone()));

                if number.ends_with('.') {
                    token_list.add(Token::Dot);
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

                let token = Token::match_reserved(&identifier);
                token_list.add(token);
                // println!("{} {} null", token.to_string(), identifier);
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

    token_list.add(Token::EOF);

    result
}
