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
    let mut model: Option<Box<Chain>> = None;
    do_while_input(|input| {
        let cmd_args: Vec<&str> = input.trim().split(' ').collect();
        match cmd_args.get(0) {
            Some(&"new") => {
                model = Some(Box::new(Chain::default()));
                println!("Now using a chain model, congratulations King xoxo");
            },
            Some(&"feed") => feed(&cmd_args, &mut model, &CharTokenizer),
            Some(&"quit") => return None,
            Some(&"") => {
                if let Some(ref mut model_inner) = model {
                    if !model_inner.is_calculated() {
                        model_inner.calculate();
                    }
                    println!("{}", model_inner.generate());
                }
            },
            _ => {}
        };
        Some(())
    });
}

fn feed<T>(cmd_args: &[&str], model: &mut Option<Box<Chain>>, tokenizer: &T)
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