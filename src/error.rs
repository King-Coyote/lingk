use std::fmt;
use std::error;
use serde_json;

use crate::token::Token;

#[derive(Debug)]
pub enum Error {
    MissingArg(&'static str),
    NoModel,
    ModelError(&'static str),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Tokenize(String, &'static str),
    TokenNotFound(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            MissingArg(arg) => 
                write!(f, "argument {} not provided", arg),
            NoModel =>
                write!(f, "no model is currently loaded. Please load or initialise a model"),
            ModelError(descr) =>
                write!(f, "Error in model: {}", descr),
            Serde(e) =>
                write!(f, "{}", e),
            Io(e) =>
                write!(f, "{}", e),
            ParseInt(e) =>
                write!(f, "could not parse param as int - {}", e),
            Tokenize(string, reason) =>
                write!(f, "Error tokenizing string {}. Reason: {}", string, reason),
            TokenNotFound(token) =>
                write!(f, "Token not found: {}", token.to_string()),    
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Serde(ref e) => Some(e),
            Io(ref e) => Some(e),
            ParseInt(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}