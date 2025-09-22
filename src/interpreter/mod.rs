pub mod token;
pub use token::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interpreter {
    text: String,
    pos: usize,
    current_token: Option<Token>,
    current_char: Option<char>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            text: String::new(),
            pos: 0,
            current_token: None,
            current_char: None,
        }
    }

    pub fn error(&self, message: &str) -> ! {
        panic!("Error parsing input: {}", message);
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() {
            self.current_char = None;
            return;
        }
        self.current_char = self.text.chars().nth(self.pos);
    }

    pub fn integer(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    pub fn get_next_token(&mut self) -> Token {
        // End of input
        if self.pos >= self.text.len() {
            return Token::new(TokenType::Eof, None);
        }

        // Skip whitespace
        if self.current_char.unwrap().is_whitespace() {
            self.advance();
            return self.get_next_token();
        }

        if self.current_char.unwrap().is_digit(10) {
            return Token::new(TokenType::Integer, Some(self.integer()));
        }

        if self.current_char == Some('+') {
            self.advance();
            return Token::new(
                TokenType::Plus,
                Some(self.current_char.unwrap().to_string()),
            );
        }
        if self.current_char == Some('-') {
            self.advance();
            return Token::new(
                TokenType::Minus,
                Some(self.current_char.unwrap().to_string()),
            );
        }
        self.error("Unexpected character");
    }

    pub fn eat(&mut self, token_type: TokenType) {
        if let Some(ref current_token) = self.current_token {
            if current_token.token_type == token_type {
                self.current_token = Some(self.get_next_token());
            } else {
                self.error("Unexpected token");
            }
        } else {
            self.error("Unexpected end of input");
        }
    }

    pub fn expr(&mut self, text: String) -> i32 {
        self.text = text;
        self.pos = 0;
        self.current_char = self.text.chars().nth(self.pos);
        self.current_token = Some(self.get_next_token());

        let left = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);

        let op = self.current_token.clone().unwrap();
        if op.token_type == TokenType::Plus {
            self.eat(TokenType::Plus);
        } else if op.token_type == TokenType::Minus {
            self.eat(TokenType::Minus);
        } else {
            self.error("Unexpected operator");
        }

        let right = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);

        if let (Some(left_val), Some(right_val)) = (left.value, right.value) {
            let left_int: i32 = left_val.parse().unwrap();
            let right_int: i32 = right_val.parse().unwrap();
            if op.token_type == TokenType::Minus {
                return left_int - right_int;
            } else {
                return left_int + right_int;
            }
        }
        self.error("Invalid expression");
    }
}
