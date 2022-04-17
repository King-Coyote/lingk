use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use token::Tokenizer;

use crate::token::CharTokenizer;
use crate::util::*;
use crate::chain::Chain;
use crate::error::Error;

pub mod token;
pub mod util;
pub mod chain;
pub mod error;

fn main() {
    println!("Welcome to lingk, a markov chain language-imitating... thing.");
    let mut model: Option<Box<Chain>> = None;
    do_while_input(|input| {
        let cmd_args: Vec<&str> = input.trim().split(' ').collect();
        match *cmd_args.get(0).unwrap_or(&"") {
            "new" => {
                model = Some(Box::new(Chain::default()));
                println!("New chain model initialised, congratulations King xoxo");
            },
            "feed" => {
                if let Err(e) = feed(&cmd_args, &mut model, &CharTokenizer) {
                    println!("feeding failed: {}", e);
                }
            },
            "file" => {
                if let Err(e) = file(&cmd_args, &mut model) {
                    println!("file reading failed: {}", e);
                }
            },
            "gen" => {
                if let Err(e) = generate_n(&cmd_args, &mut model) {
                    println!("Generation failed: {}", e);
                }
            },
            "save" => {
                match save(&cmd_args, &model) {
                    Err(e) => println!("Saving failed: {}", e),
                    Ok(()) => println!("Saved successfully!")
                };
            },
            "load" => {
                match load(&cmd_args) {
                    Ok(loaded) => {
                        model = Some(loaded);
                        println!("Loaded model successfully!");
                    },
                    Err(e) => println!("Loading failed: {}", e),
                };
            },
            "quit" => return None,
            "" => {
                if let Some(ref mut model_inner) = model {
                    if !model_inner.is_calculated() {
                        model_inner.calculate();
                    }
                    println!("{}", model_inner.generate());
                }
            },
            _ => println!("Unrecognized command.")
        };
        Some(())
    });
}

fn save(cmd_args: &[&str], model: &Option<Box<Chain>>) -> Result<(), Error> {
    let filename = cmd_args.get(1).ok_or(Error::MissingArg("filename"))?;
    let model = model.as_ref().ok_or(Error::NoModel)?;
    let json = serde_json::to_string(&model)?;
    fs::write(filename, json)?;
    Ok(())
}

fn load(cmd_args: &[&str]) -> Result<Box<Chain>, Error> {
    let filename = cmd_args.get(1).ok_or(Error::MissingArg("filename"))?;
    let data = fs::read_to_string(&filename)?;
    let model = serde_json::from_str(&data)?;
    Ok(model)
}

fn generate_n(cmd_args: &[&str], model: &mut Option<Box<Chain>>) -> Result<(), Error> {
    let n = cmd_args.get(1)
        .map(|arg| arg.parse::<i32>())
        .ok_or(Error::MissingArg("num to generate"))??;
    if let Some(ref mut model_inner) = model {
        for _ in 0..n {
            if !model_inner.is_calculated() {
                model_inner.calculate();
            }
            println!("{}", model_inner.generate());
        }
    }
    Ok(())
}

fn file(cmd_args: &[&str], model: &mut Option<Box<Chain>>) -> Result<(), Error> {
    let path = Path::new(cmd_args.get(1).ok_or(Error::MissingArg("filename"))?);
    let model = model.as_mut().ok_or(Error::NoModel)?;
    let file = File::open(path)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    for line in lines {
        model.feed(CharTokenizer.tokenize(&line));
    }
    println!("File read successfully.");
    Ok(())
}

fn feed<T>(cmd_args: &[&str], model: &mut Option<Box<Chain>>, tokenizer: &T) -> Result<(), Error>
where
    T: Tokenizer
{
    let data = cmd_args.get(1).ok_or(Error::MissingArg("data"))?;
    let model = model.as_mut().ok_or(Error::NoModel)?;
    model.feed(tokenizer.tokenize(data));
    Ok(())
}