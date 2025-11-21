//! `lexer` is a crate containing functions and data types useful for lexing (aka lexicalising) a
//! program. The [lexer] function and [Token] enum are the primary public interfaces to this crate.
use std::{fmt,process::ExitCode};
use log::{trace, debug, info,warn, error};



/// Enum of all valid tokens
pub enum Token {
    ReturnTok,
    IntTok(i32),
    Period,
    The,
    Value,
}
impl fmt::Debug for Token {
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



/// This function takes in a string representing the contents of the sourcecode and returns a
/// result with either an error code or a vector of tokens.
pub fn lexer(s: String) -> Result<Vec<Token>, ExitCode> {
    info!("Starting lexer");

    // convert s to a vector of strings
    let exploded: Vec<&str> = s.split_whitespace().collect();
    trace!("Exploded file contents:\n{:?}", exploded);

    let mut tokens: Vec<Token> = vec![];
    match lex_helper(&exploded[..] , &mut tokens) {
        Ok(_) => Ok(tokens),
        Err(b) => Err(b)
    }
}



/// This function recursively traverses the vector of strings, lexing each word. It is mutually
/// tail recursive with [lex_num]
fn lex_helper<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {

    let word = s.first();

    debug!("Running lex_helper on: {:?} ", word);
    trace!("s = {:?}, tokens = {:?}", s, tokens);

    match word {
        None => return Ok(&*tokens),                                                //empty slice
        Some(first) if first.is_empty() => (),                               //empty string
        Some(&"return")  => tokens.push(Token::ReturnTok),                          //Return
        Some(&"the") => tokens.push(Token::The),                                    //The
        Some(&"value") => tokens.push(Token::Value),                                //Value
        Some(&".") => tokens.push(Token::Period),                                   //Period  WARN: period must have space before it as of now
        Some(first) => match first.chars().next() {                          //other
            Some(c) if c.is_ascii_digit() => return lex_num(s,tokens),          //starts with number
            //Some(c) if c.is_ascii_alphabetic() => lex_word(s,tokens),                 //starts with letter
            //Some('*') =>                                                              //starts with star
            _ => {error!("Unknown token: {:?}", word); return Err(ExitCode::from(10))}//unimplemented
        }
    }
    lex_helper(&s[1..], tokens)
}



/// This function lexes words that start with digits.
fn lex_num<'a>(s: &[&str], tokens: &'a mut Vec<Token>) -> Result<&'a Vec<Token>, ExitCode> {

    //! The function adds characters to the `output` string while checking for decimal points and
    //! invalid characters. `output` is then converted to an `i32` or `f64`.
    let mut output = String::from("");
    let mut is_float: bool = false;
    let word = s.first().unwrap();

    debug!("Running lex_num on: {:?} ", word);
    trace!("s = {:?}, tokens = {:?} ", s, tokens);

    for chars in word.chars() {
        match chars {
            c @ '0'..='9' => output.push(c), //push all numeric chars to output
            '.' => {
                if is_float {error!("Unknown token {:?}", word); return Err(ExitCode::from(10));} //invalid if >1 period found
                else {output.push('.'); is_float = true;} //one period is treated as decimal
            },
            ' ' => {warn!("Whitespace found in word: {:?}", word);break;},
            _ => {error!("Unknown token: {:?}", word); return Err(ExitCode::from(10))}
        }
    }

    if !is_float {
        tokens.push(Token::IntTok(output.parse().unwrap()));
    }
    else{ //WARN: floats not implemented
        error!("Floats not implemented yet!"); 
        return Err(ExitCode::from(20));
    }

    lex_helper(&s[1..], tokens) 
}
