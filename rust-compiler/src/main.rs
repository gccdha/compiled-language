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
            trace!("Successfully opened file {:?}", display);
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
    The,
    Value,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::ReturnTok => write!(f, "RETURN"),
            Token::IntTok(val) => write!(f, "INT<{:?}>", val),
            Token::Period => write!(f, "PERIOD"),
            Token::The => write!(f, "THE"),
            Token::Value => write!(f, "VALUE"),
        }
    }
}

struct PreTree<'p>(&'p Vec<Token>);
impl fmt::Display for PreTree <'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.0 {
            match write!(f, "{}", token) {
                Ok(_) => continue,
                Err(a) => return Err(a)
            }
        }
        return Ok(());
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
//
//TODO: make sure that characters at the end of strings are accounted for
//
//man recursive lexing routine
fn lex_helper<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
    let word = s.first();
    debug!("Running lex_helper on: {:?} ", word);
    trace!("s = {:?}, tokens = {}", s, PreTree(tokens));
    match word {
        None => Ok(tokens),                                                    //empty slice
        Some(first) if first.is_empty() => lex_helper(&s[1..], tokens), //empty string
        //Some(first) if first.starts_with("**") =>,                             //2 or 3 stars
        Some("return")  => tokens.push(Token::ReturnTok),
        Some("the") => tokens.push(Token::The), 
        Some("value") => tokens.push(Token::Value),
        Some(first) => match first.chars().next() {
            Some(c) if c.is_ascii_digit() => lex_num(s,tokens),          //starts with number
            //Some(c) if c.is_ascii_alphabetic() => lex_word(s,tokens),    //starts with letter
            //Some('*') =>                                                       //starts with star
            _ => Err(ExitCode::from(20))                                       //unimplemented
        }
    }
    lex_helper(&s[1..], tokens)
}

//function to lex strings starting with numbers
fn lex_num<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
    let mut output = String::from("");
    let mut is_float: bool = false;
    let word = s.first().unwrap();

    debug!("Running lex_num on: {} ", word);
    trace!("s = {:?}, tokens = {} ", s, PreTree(tokens));

    for chars in word.chars() {
        match chars {
            c @ '0'..='9' => output.push(c),
            '.' => {
                if is_float {return Err(ExitCode::from(10));} //invalid if >1 period found
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
    lex_helper(&s[1..], tokens)
}

//WARN: maybe i just need to return &mut Vec<Token>? Check what the difference is...
//
// //function to lex words that are not keywords (keywords are handled in lex_helper)
// fn lex_word<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {
//     match *s.first().unwrap() {
//
//         _ => return Err(ExitCode::from(20))
//     }
//     lex_helper(&s[1..], tokens)
// }

//TODO: Testing
//TODO: Lexer
//TODO: Parser
