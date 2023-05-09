use crate::tools::token::*;
use crate::tools::lexer::*;

pub struct BasicLexer {
    input: String
}

impl BasicLexer {
    /// Creates a new [`BasicLexer`].
    pub fn new(in_input: String) -> Self {
        BasicLexer { input: in_input }
    }
}

impl Lexer for BasicLexer {
    fn lex(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut current_pos: usize = 0;
        let mut token_start;
        while current_pos < self.input.len() {

            token_start = current_pos;
            let ahead = self.input.chars().nth(current_pos).unwrap();
            if ahead.is_whitespace() {
                current_pos += 1;
            } else if ahead == '+' {
                current_pos += 1;
                tokens.push(
                    Token::new(
                        Type::SUM,
                         ahead.to_string(),
                          token_start
                        )
                    )

            } else if ahead == '*' {
                current_pos += 1;
                tokens.push(
                    Token::new(
                        Type::PRODUCT,
                         ahead.to_string(),
                          token_start
                        )
                    )
            } else if ahead.is_numeric() {
                let mut text: String = String::new();
                while current_pos < self.input.len() && self.input.chars().nth(current_pos).unwrap().is_numeric() {
                    text = text + self.input.chars().nth(current_pos).unwrap().to_string().as_str();
                    current_pos += 1;
                }

                tokens.push(
                    Token::new(
                        Type::NUM,
                        text,
                        token_start
                    )
                )
                
            } else if ahead.is_alphabetic() {

                let mut text: String = String::new();
                while current_pos < self.input.len() && self.input.chars().nth(current_pos).unwrap().is_alphabetic() {
                    text = text + self.input.chars().nth(current_pos).unwrap().to_string().as_str();
                    current_pos += 1;
                }

                let tpe = match text.as_str() {
                    "true" => Type::TRUE,
                    "false" => Type::FALSE,
                    _ => Type::ID
                };

                tokens.push(
                    Token::new(
                        tpe,
                        text,
                        token_start
                    )
                )

            } else {
                println!("{} is at position {}", ahead, current_pos);
            }

        }
        
        tokens.push(
            Token::new(
                Type::EOF,
                "<EOF>".into(),
                current_pos
            )
        );

        return tokens;
    }
}