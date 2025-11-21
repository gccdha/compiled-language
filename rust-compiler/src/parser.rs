//! `parser` is a crate containing functions and data types useful for parsing the Vec of tokens
//! output by [crate::lexer::lexer]. The [parser] function and [Expr] enum are the primary
//! interfaces to this crate.
use std::{fmt,process::ExitCode};
use log::{debug, info}; //set err level using `export RUST_LOG=<level>`
use crate::lexer::Token;

//TODO: gonna need to split up Expr into more enums for datatypes and whatnot later.
/// This enum contains every expression type in the language
pub enum Expr{
    Return(Box<Expr>),
    Const(i32),
    Null,
}
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Return(a) => write!(f,"Return({:?})",a),
            Expr::Const(a) => write!(f,"Const({:?})",a),
            Expr::Null => write!(f, "Null"),
        }
    }
} 


/// This struct mainly exists to mark when the AST is complete compared to just an Expr. I may
/// remove it later, or add more functionality in the form of a pretty-print.
pub struct AST(pub Expr);
impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { AST(exp) => write!(f, "AST: {:?}",exp)} 
    }
}


/// This function takes in a vector of tokens and returns an AST or error code.
pub fn parser(tokens: Vec<Token>) -> Result<AST,ExitCode>{
    info!("Parser started");
    //let ast = AST(Expr::Null);
    let mut tok_iter = tokens.iter();
    Ok(AST(parser_helper(&mut tok_iter)))
}



/// This function takes an iterator of tokens and 
fn parser_helper<'a>(tokens: &mut impl Iterator<Item=&'a Token>) -> Expr {
    let t = tokens.next();
    debug!("parsing token:{:?}",t );
    match t {
        None => Expr::Null, //iteration done
        Some(tok) => match tok {
            Token::IntTok(i) => Expr::Const(*i),
            Token::Period => parser_helper(tokens), //for now i think i can just ignore period
            Token::ReturnTok => Expr::Return(Box::new(parser_helper(tokens))),
            Token::The => parser_helper(tokens), //we can ignore for now but in the future we want to enforce english grammar
            Token::Value => parser_helper(tokens), //same for "The" 
        }, 
    } 
    //return Expr::Null;
}
