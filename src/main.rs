#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Integer,
    Plus,
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>) -> Self {
        Token { token_type, value }
    }
}

pub struct Interpreter {
    text: String,
    pos: usize,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            text: String::new(),
            pos: 0,
            current_token: None,
        }
    }

    pub fn error(&self) -> ! {
        panic!("Error parsing input");
    }

    pub fn get_next_token(&mut self) -> Token {
        let text = self.text.clone();
        if self.pos >= text.len() {
            return Token::new(TokenType::Eof, None);
        }
        let current_char = text.chars().nth(self.pos).unwrap();
        if current_char.is_digit(10) {
            self.pos += 1;
            return Token::new(TokenType::Integer, Some(current_char.to_string()));
        }
        if current_char == '+' {
            self.pos += 1;
            return Token::new(TokenType::Plus, Some(current_char.to_string()));
        }
        self.error();
    }

    pub fn eat(&mut self, token_type: TokenType) {
        if let Some(ref current_token) = self.current_token {
            if current_token.token_type == token_type {
                self.current_token = Some(self.get_next_token());
            } else {
                self.error();
            }
        } else {
            self.error();
        }
    }

    pub fn expr(&mut self, text: String) -> i32 {
        self.text = text;
        self.pos = 0;
        self.current_token = Some(self.get_next_token());

        let left = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);

        let _op = self.current_token.clone().unwrap();
        self.eat(TokenType::Plus);

        let right = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);

        if let (Some(left_val), Some(right_val)) = (left.value, right.value) {
            let left_int: i32 = left_val.parse().unwrap();
            let right_int: i32 = right_val.parse().unwrap();
            return left_int + right_int;
        }
        self.error();
    }
}

use std::io::Write;

fn main() {
    loop {
        let mut interpreter = Interpreter::new();
        let mut input = String::new();
        print!("calc> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let result = interpreter.expr(input.trim().to_string());
        println!("{}", result);
    }
}
