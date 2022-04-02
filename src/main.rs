use std::io;

use token::Tokenizer;

use crate::token::CharTokenizer;
use crate::util::*;
use crate::chain::Chain;

pub mod token;
pub mod util;
pub mod chain;

fn main() {
    println!("Welcome to lingk, a markov chain language-imitating... thing.");
    let mut model: Option<Chain> = None;
    do_while_input(|input| {
        let cmd_args: Vec<&str> = input.trim().split(' ').collect();
        match cmd_args.get(0) {
            Some(&"new") => {
                model = Some(Chain::default());
                println!("Now using a chain model, congratulations King xoxo");
            },
            Some(&"feed") => feed(&cmd_args, &mut model, &CharTokenizer),
            Some(&"quit") => return None,
            _ => {}
        };
        Some(())
    });
}

fn feed<T>(cmd_args: &[&str], model: &mut Option<Chain>, tokenizer: &T)
where
    T: Tokenizer
{                
    if cmd_args.get(1).is_none() {
        println!("No string provided. Please provide a string for feeding.");
    } else if let Some(ref mut model_inner) = model {
        let data = cmd_args.get(1).unwrap();
        model_inner.feed(tokenizer.tokenize(data));
    } else {
        println!("No model is currently loaded. Please load or initialise a model before feeding.");
    }
}