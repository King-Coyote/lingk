
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Token {
    Start,
    Char(char),
    Str(String),
    End,
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