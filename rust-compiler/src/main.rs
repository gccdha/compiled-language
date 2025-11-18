use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;
use log::{trace, debug, info, error}; //set err level using `export RUST_LOG=<level>`
mod lexer;
mod parser;
use crate::lexer::*;
use crate::parser::*;
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
    let path = Path::new("stage_1.md");
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



    match lexer(s) {
        Ok(tokens) => {
            info!("Lexer finished");
            debug!("Tokens: {}", PreTree(&tokens));
        },
        Err(code) => {
            error!("Error in lexing. Exit code: {:?}", code);
            return code;
        }
    }
    return ExitCode::SUCCESS;
}

//TODO: Testing

//TODO: Parser
