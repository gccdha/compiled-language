use std::fmt;
use std::process::ExitCode;
use log::{trace, debug, info, error}; //set err level using `export RUST_LOG=<level>`
use crate::lexer::PreTree;

enum Expr{
    Return(Box<Expr>),
    Const(i32),
    Null,
}

pub struct AST(Expr);

//TODO: implement fmt::Display atribute for AST and Expr

pub fn parser(tokens: PreTree) -> AST{
    let ast = AST(Expr::Null);
    parser_helper(tokens, ast)
}

fn parser_helper(tokens: PreTree, ast: AST) -> AST {
    return AST(Expr::Null);
}
