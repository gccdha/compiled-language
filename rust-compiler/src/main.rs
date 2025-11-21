use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;
use log::{trace, debug, info, error}; //NOTE: set err level using `export RUST_LOG=<level>`
mod lexer;
mod parser;
mod codegen;
use crate::lexer::*;
use crate::parser::*;
use crate::codegen::*;
/*
* Exit codes:
* 0 -> good
* 10 -> unknown token
* 20 -> unimplemented
* 64 -> invalid options
* 66 -> error finding or reading file
* */


fn main() -> ExitCode {


    //start logging
    env_logger::init();
    info!("Compiler started");


    //TODO: write command line argument parsing function once you have better rust knowledge
    let path = Path::new("stage_1.md"); //input file
    let display = path.display();
    let output  = String::from("stage_1"); //output file


    //open file
    debug!("Opening file {:?}", display);
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


    //extract file contents as string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            error!("couldn't read {:?}: {:?}", display, why);
            return ExitCode::from(66);
        },
        Ok(_) => (),
    }


    //pass string to lexer
    let lexed = match lexer(s) {
        Ok(tokens) => {
            info!("Lexer finished");
            trace!("Tokens: {:?}", tokens);
            tokens
        },
        Err(code) => {
            error!("Error in lexing. Exit code: {:?}", code);
            return code;
        }
    };

    
    //pass tokens to parser
    let ast = match parser(lexed){
        Ok(tree) => {
            info!("Parser finished");
            debug!("AST:{:?}", tree);
            tree
        }, 
        Err(code) => {
            error!("Error in parsing. Exit code: {:?}", code);
            return code;
        }
    };


    //pass AST to codegen
    codegen(ast, output);


    info!("Compilation complete!");
    return ExitCode::SUCCESS;
}

//TODO: Testing

//TODO: Parser
