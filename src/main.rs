use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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
            Some(&"file") => file(&cmd_args, &mut model),
            Some(&"gen") => generate_n(&cmd_args, &mut model),
            Some(&"save") => {
                match save(&cmd_args, &model) {
                    Err(error) => println!("Saving failed: {}", error),
                    Ok(()) => println!("Saved successfully!")
                };
            },
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

fn save(cmd_args: &[&str], model: &Option<Box<Chain>>) -> Result<(), String> {
    let filename = cmd_args.get(1).ok_or("filename not provided")?;
    let model = model.as_ref().ok_or("no model provided")?;
    let json = serde_json::to_string(&model).map_err(|e| e.to_string())?;
    fs::write(filename, json).map_err(|e| e.to_string())?;
    Ok(())
}

fn generate_n(cmd_args: &[&str], model: &mut Option<Box<Chain>>) {
    //defaulting to 1 because I am lazy and need to refactor all these fns
    let n = cmd_args.get(1)
        .map(|arg| arg.parse::<i32>().unwrap_or(1))
        .or(Some(1))
        .unwrap();
    if let Some(ref mut model_inner) = model {
        for _ in 0..n {
            if !model_inner.is_calculated() {
                model_inner.calculate();
            }
            println!("{}", model_inner.generate());
        }
    }
}

fn file(cmd_args: &[&str], model: &mut Option<Box<Chain>>) {
    if cmd_args.get(1).is_none() {
        println!("No filename provided. Please provide a file for feeding.");
        return;
    }
    if model.is_none() {
        println!("No model is currently loaded. Please load or initialise a model before feeding.");
        return;
    }
    let model_inner = model.as_mut().unwrap();
    let path = Path::new(cmd_args.get(1).unwrap());
    let file = File::open(path);
    if let Ok(lines) = file.map(|f| BufReader::new(f).lines()) {
        for line in lines.filter(|l| l.is_ok()).map(|l| l.unwrap()) {
            model_inner.feed(CharTokenizer.tokenize(&line));
        }
    } else {
        println!("File does not exist. u idiot. u rascal");
    }
    println!("File read successfully.");
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