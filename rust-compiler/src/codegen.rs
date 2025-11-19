//TODO: compress the use statements a bit
use std::fmt;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::process::Command;
use std::process::ExitCode;
use log::{trace, debug, info,warn, error};
use crate::lexer::*;
use crate::parser::*;


//WARN: is this the best way to do this? does it really need to be a byte slice instead of a String
//slice?
static ASM_HEADER: &'static [u8; 28] = b".text\n.globl _start\n_start:\n";
static ASM_SYSCALL: &'static [u8; 7] = b"syscall";

//TODO: make it so that if you are using trace mode the asm and .o files are saved
//also, look into the "?" opperator for propegating error up the call stack
//
//TODO: comment, debug messages etc.
pub fn codegen(ast: AST, output: String) {
    info!("Starting codegen");

    let mut file = OpenOptions::new() //TODO: make it not append if file exists
        .append(true)
        .create(true)
        .open(output.clone() + ".s" )
        .expect("Couldn't open assembly file to write.");
    file.write_all(ASM_HEADER).expect("error writing ASM_HEADER");
    let AST(tree) = ast;
    asmgen(tree, &mut file);
    file.write_all(ASM_SYSCALL).expect("error writing ASM_SYSCALL");
    assemble(output);
}

fn asmgen(ast: Expr, output: &mut File) {
    trace!("AST: {}", ast);
    match ast {
        Expr::Return(e) => {
            output.write_all(b"mov $60, %rax\n").expect("error writing for Expr::Return");
            asmgen(*e, output); //maybe add ASM_SYSCALL after this?
        },
        Expr::Const(n) => {
            let line = format!("mov ${}, %rdi\n", n);
            output.write_all(line.as_bytes()).expect("error writing for Expr::Const");
        },
        Expr::Null => warn!("Null found in AST during assembly generation")
    }
}

fn assemble(filename: String){
    debug!("Assembling program");
    let asm_file = format!("{}.s", filename);
    let obj_file = format!("{}.o", filename);

    Command::new("as")
        .args(&["--64","-o",&obj_file, &asm_file])
        .status()
        .expect("Error assembling program");

    debug!("Linking program");
    Command::new("ld")
        .args(&["-o",&filename, &obj_file])
        .status()
        .expect("Error linking program");

}
