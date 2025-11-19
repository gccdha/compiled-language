use std::fmt;
use std::process::ExitCode;
use log::{trace, debug, info, error}; //set err level using `export RUST_LOG=<level>`
use crate::lexer::{PreTree,Token};

pub enum Expr{
    Return(Box<Expr>),
    Const(i32),
    Null,
}

pub struct AST(pub Expr);

pub fn parser(tokens: Vec<Token>) -> AST{
    //let ast = AST(Expr::Null);
    let mut tok_iter = tokens.iter();
    AST(parser_helper(&mut tok_iter))
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Return(a) => write!(f,"Return({})",a),
            Expr::Const(a) => write!(f,"Const({})",a),
            Expr::Null => write!(f, "Null"),
        }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { AST(exp) => write!(f, "AST: {}",exp)} 
    }
}

fn parser_helper<'a>(tokens: &mut impl Iterator<Item=&'a Token>) -> Expr {
    //TODO: Make sure that .next() works like you think it does i.e. it consumes each item that 
    //it iterates over, so you can just pass `tokens` until .next() returns None
    let t = tokens.next();
    debug!("parsing token:{:?}",t );
    match t {

        None => todo!(), //iteration done
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
