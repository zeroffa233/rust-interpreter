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
    // 初始化函数
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

    // lexer 部分，只负责提供把字节流转为 token 的相关方法
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

    // parser/interpreter 部分，负责识别结构和按照结构进行结果的生成

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

    // 主要变动 1：增加可以根据当前 token 返回数字并“吃掉”数字的函数
    pub fn term(&mut self) -> i32 {
        let token = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);
        return token.value.unwrap().parse::<i32>().unwrap();
    }

    // 主要变动 2：修改 expr 函数，先初始化第一个数字 Token，然后循环“吃掉”运算符和数字，更新结果。
    pub fn expr(&mut self, text: String) -> i32 {
        self.text = text;
        self.pos = 0;
        self.current_char = self.text.chars().nth(self.pos);
        self.current_token = Some(self.get_next_token());

        let mut result = self.term();

        while let Some(ref token) = self.current_token.clone() {
            if token.token_type == TokenType::Plus {
                self.eat(TokenType::Plus);
                result += self.term();
            } else if token.token_type == TokenType::Minus {
                self.eat(TokenType::Minus);
                result -= self.term();
            } else {
                break;
            }
        }
        result
    }
}
