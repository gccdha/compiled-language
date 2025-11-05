use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;
use log::{trace, debug, info, error, warn}; //set err level using `export RUST_LOG=<level>`
/*
* Exit codes:
* 0 -> good
* 64 -> invalid options
* 66 -> error finding or reading file
* */


fn main() -> ExitCode {
    env_logger::init();
    info!("Compiler started");

    //TODO: write command line argument parsing function once you have better rust knowledge
    let path = Path::new("main.md");
    let display = path.display();

    //open file
    trace!("Opening file {}", display);
    let mut file = match File::open(&path) {
        Err(why) => {
            error!("couldn't open {:?}: {:?}", display, why);
            return ExitCode::from(66);
            },
        Ok(file) => {
            trace!("Successfully opened file {}", display);
            file
        },
    };

    //extract contents as string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            error!("couldn't read {:?}: {:?}", display, why);
            return ExitCode::from(66);
        },
        Ok(_) => (),
    }


    return ExitCode::SUCCESS;
}


enum Token {
    ReturnTok,
    IntTok(i32),
    Period
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::ReturnTok => write!(f, "RETURN"),
            Token::IntTok(val) => write!(f, "INT<{}>", val),
            Token::Period => write!(f, "PERIOD"),
        }
    }
}


fn lexer(s: String) -> Vec<Token> {
    info!("Starting lexer");
    let exploded: Vec<char> = s.chars().collect(); //READ: not exactly sure why we need all these methods
    lex_helper(exploded.as_slice(), vec![])
}

fn lex_helper(s: &[char], tokens: Vec<Token>) -> Vec<Token> {
    match s {
        ['0'..='9', rest @ ..] => lex_num(s, tokens), //READ: understand syntax
        ['a'..='z' | 'A'..='Z', rest @ ..] => lex_word(s, tokens),
        ['.', rest @ ..] => tokens.push(Token::Period)
    }
}

fn lex_num(s: &[char], tokens: Vec<Token>) -> Token {
    
}

fn lex_word(s: &[char], tokens: Vec<Token>) -> Token {
}

//TODO: Testing
//TODO: Lexer
//TODO: Parser
