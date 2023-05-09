use crate::tools::token::Token;

pub trait Lexer {
    fn lex(&self) -> Vec<Token>;
}