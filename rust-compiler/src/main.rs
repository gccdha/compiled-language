use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;
use log::{trace, debug, info, error, warn}; //set err level using `export RUST_LOG=<level>`
/*
* Exit codes:
* 0 -> good
* 10 -> unknown token
* 20 -> unimplemented
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
    Period,
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


fn lexer(s: String) -> Result<Vec<Token>, ExitCode> {
    info!("Starting lexer");
    let exploded: Vec<&str> = s.split_whitespace().collect(); //READ: not exactly sure why we need all these methods (Could i just use a string slice 🤔)
    let mut tokens: Vec<Token> = vec![];
    match lex_helper(&exploded[..] , &mut tokens) {
        Ok(_) => Ok(tokens),
        Err(b) => Err(b)
    }
}


//TODO: make sure we are handling newline propperly and also tab characters
//TODO: comment and logging
//
//TODO: make sure that characters at the end of strings are accounted for
fn lex_helper<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
    match s.first() {
        None => Ok(tokens), //empty slice
        Some(first) if first.is_empty() => lex_helper(&s[1..], tokens), //empty string
        //Some(first) if first.starts_with("**") =>, //2 or 3 stars
        Some(first) => match first.chars().next() {//WARN: why .next()?
//
            Some(c) if c.is_ascii_digit() => lex_num(s,tokens), //starts with is_ascii_digit
            Some(c) if c.is_ascii_alphabetic() => lex_word(s,tokens), //letter
            //Some('*') => //star
            _ => Err(ExitCode::from(20))
        }
    }
}

//TODO: comment and add logging
fn lex_num<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
    let mut output = String::from("");
    let mut is_float: bool = false;
    for chars in s.first().unwrap().chars() {
        match chars {
            c @ '0'..='9' => output.push(c),
            '.' => {
                if is_float {return Err(ExitCode::from(10));}
                else {output.push('.'); is_float = true;}
            },
            ' ' => break, //NOTE: maybe do something like is_ascii_whitespace?
            _ => return Err(ExitCode::from(10)),
        }
    }

    if !is_float {
        tokens.push(Token::IntTok(output.parse().unwrap()));
    }
    else{
        error!("floats not implemented yet!");
        return Err(ExitCode::from(20));
    }
    let (_, rest) = s.split_at(output.len());
    lex_helper(rest, tokens)
}

//WARN: maybe i just need to return &mut Vec<Token>? Check what the difference is...
fn lex_word<'a>(s: &[char], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
    match s {
        "first".split() => tokens.push(Token::ReturnTok),
        _ => return Err(ExitCode::FAILURE)
    }
    lex_helper(s, tokens)
}

//TODO: Testing
//TODO: Lexer
//TODO: Parser
