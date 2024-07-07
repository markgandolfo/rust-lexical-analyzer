pub struct TokenList {
    tokens: Vec<Token>,
}

impl TokenList {
    pub fn new() -> Self {
        TokenList { tokens: Vec::new() }
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }

    #[allow(dead_code)]
    pub fn print_all(&self) {
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }
}

pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Plus,
    Minus,
    Dot,
    Comma,
    SemiColon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    And,
    Class,
    Else,
    EOF,
    False,
    For,
    Fun,
    If,
    Nil,
    Number(String),
    Or,
    Print,
    Return,
    String(String),
    Super,
    This,
    True,
    Var,
    While,
    Identifier(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::LeftParen => "LEFT_PAREN ( null".to_string(),
            Token::RightParen => "RIGHT_PAREN ) null".to_string(),
            Token::LeftBrace => "LEFT_BRACE { null".to_string(),
            Token::RightBrace => "RIGHT_BRACE } null".to_string(),
            Token::Star => "STAR * null".to_string(),
            Token::Plus => "PLUS + null".to_string(),
            Token::Minus => "MINUS - null".to_string(),
            Token::Dot => "DOT . null".to_string(),
            Token::Comma => "COMMA , null".to_string(),
            Token::SemiColon => "SEMICOLON ; null".to_string(),
            Token::Equal => "EQUAL = null".to_string(),
            Token::EqualEqual => "EQUAL_EQUAL == null".to_string(),
            Token::Bang => "BANG ! null".to_string(),
            Token::BangEqual => "BANG_EQUAL != null".to_string(),
            Token::Greater => "GREATER > null".to_string(),
            Token::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
            Token::Less => "LESS < null".to_string(),
            Token::LessEqual => "LESS_EQUAL <= null".to_string(),
            Token::Slash => "SLASH / null".to_string(),
            Token::And => "AND and null".to_string(),
            Token::Class => "CLASS class null".to_string(),
            Token::Else => "ELSE else null".to_string(),
            Token::EOF => "EOF  null".to_string(),
            Token::False => "FALSE false null".to_string(),
            Token::For => "FOR for null".to_string(),
            Token::Fun => "FUN fun null".to_string(),
            Token::If => "IF if null".to_string(),
            Token::Number(number) => {
                let mut display_number = number.clone();
                let mut number = number.clone();

                if number.ends_with('.') {
                    display_number.push('0');
                    number.pop();
                } else if !number.contains('.') {
                    display_number.push_str(".0");
                }

                display_number = Token::truncate_zeros(&display_number);
                format!("NUMBER {} {}", number, display_number)
            }
            Token::Nil => "NIL nil null".to_string(),
            Token::Or => "OR or null".to_string(),
            Token::Print => "PRINT print null".to_string(),
            Token::Return => "RETURN return null".to_string(),
            Token::Super => "SUPER super null".to_string(),
            Token::String(s) => format!("STRING \"{}\" {}", s, s),
            Token::This => "THIS this null".to_string(),
            Token::True => "TRUE true null".to_string(),
            Token::Var => "VAR var null".to_string(),
            Token::While => "WHILE while null".to_string(),
            Token::Identifier(s) => format!("IDENTIFIER {} null", s),
        }
    }

    pub fn match_reserved(identifier: &str) -> Token {
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
            _ => Token::Identifier(identifier.to_string()),
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
}
