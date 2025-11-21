//! `codegen` is a crate that contains the code generation functions for this assembler. This
//! includes generation of assembly from the AST as well as assembling and linking the executable
//! using binutils' `as` and `ld`. [codegen] is the only public method.
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::process::{Command,ExitCode};
use log::{trace, debug, info,warn, error};
use crate::parser::*;


static ASM_HEADER: &'static [u8; 28] = b".text\n.globl _start\n_start:\n";
static ASM_SYSCALL: &'static [u8; 7] = b"syscall";

/// This function opens and writes to an assembly file the results of evaluating the AST.
pub fn codegen(ast: AST, output: String) -> Result<(),ExitCode> {
    info!("Starting codegen");

    //open assembly file for writin
    //FIX: Current behaviour appends to the assembly file if it exists but it should write over it.
    let mut file = match OpenOptions::new() 
        .append(true) //maybe just don't use this and save to str before write?
        .create(true)
        .open(output.clone() + ".s" ) {
            Ok(a) =>  a,
            Err(b) =>  {
                error!("Failed to open assembly file for writing. Error: {:?}", b);
                return Err(ExitCode::from(66));
            }
        };

    //Write header to file
    a_write(&mut file, ASM_HEADER)?;

    let AST(tree) = ast;
    asmgen(tree, &mut file)?;

    //Write the final "syscall" to the end of the file
    //TODO: include loading the syscall register in this, and make the return token give assembly
    //that just loads whatever value into the return register.
    a_write(&mut file, ASM_SYSCALL)?;
    a_write(&mut file, b"\n")?; // add newline to prevent assembler warning.
    assemble(output)
}


/// This function traverses the AST and generates assembly based on its structure.
fn asmgen(ast: Expr, output: &mut File) -> Result<(), ExitCode> {
    trace!("AST: {:?}", ast);
    match ast {
        Expr::Return(e) => {
            a_write(output,b"mov $60, %rax\n")?; // exit syscall -> syscall register
            asmgen(*e, output)
        },
        Expr::Const(n) => {
            let line = format!("mov ${}, %rdi\n", n); // n -> return register
            a_write(output, line.as_bytes())
        },
        Expr::Null => {
            warn!("Null found in AST during assembly generation"); 
            return Err(ExitCode::from(11));
        }
    }
}


/// This function uses the `as` and `ld` binutils tools to assemble the program
fn assemble(filename: String)-> Result<(), ExitCode> {
    //TODO: include an option to remove .s and .o files when they are no longer needed.
    let asm_file = format!("{}.s", filename);
    let obj_file = format!("{}.o", filename);

    debug!("Assembling program");
    Command::new("as")
        .args(&["--64","-o",&obj_file, &asm_file])
        .status()
        .map_err(|x| {
            error!("Error assembling file: {:?}",x);
            ExitCode::from(30)
        })?;

    debug!("Linking program");
    Command::new("ld")
        .args(&["-o",&filename, &obj_file])
        .status()
        .map_err(|x| {
            error!("Error linking program: {:?}",x);
            ExitCode::from(30)
        })?;

    return Ok(());

}


/// This is a function that writes the input bytes to a line in the assembly file.
fn a_write(output: &mut File, bytes: &[u8]) -> Result<(),ExitCode> {
    return output.write_all(bytes).map_err(|x|{ 
        error!("Error writing line: {:?}",x);
        trace!("Line = {:?}", bytes);
        ExitCode::from(74)
    });
}
