use chumsky::{primitive, recovery, text, Parser};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Ident(String),
    // This uses 'String' since we need Tokens to implement Hash for chumsky. 
    // Literals are parsed into 'f64' in the parser
    FloatLit(String),
    IntLit(i64),
    StringLit(String),
    Input,
    Let,
    Equals,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Arrow,
    Comma,
    QuestionMark,
    Colon,
    Semicolon,
}

enum ValueTo

struct LexerBuilder;

impl LexerBuilder {
    pub fn build() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        todo!()
    }
}




fn lexer (line: String) -> Vec<Token> {
    /*let vec = Vec::new();

    
    while == true {
        if == "\s" || "\n" || "\t"{
            
        }
    }
    
    return tokens;*/
    todo!()
}