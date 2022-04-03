use std::string::ToString;


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Start,
    Char(char),
    Str(String),
    End,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        use Token::*;
        match &*self {
            Start => "".to_string(),
            Char(c) => c.to_string(),
            Str(s) => s.to_string(),
            End => "".to_string(),
        }
    }
}

// simple tokenizer for characters only
pub struct CharTokenizer;

impl Tokenizer for CharTokenizer {
    fn tokenize(&self, data: &str) -> Vec<Token> {
        let mut tokens = vec![Token::Start];
        tokens.extend(data.chars().map(Token::Char));
        tokens.push(Token::End);
        tokens
    }
}

pub trait Tokenizer {
    fn tokenize(&self, data: &str) -> Vec<Token>;
}