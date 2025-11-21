# About
This project is a toy compiled language. The goals of this project are to

- Create a compiled programming language with syntax and semantics that allows it to be read as english
text in a markdown viewer.
- Improve my understanding of compilers and the mechanisms that they use
- Get better at Rust and Assembly
- Tentatively, I also want to bootstrap a compiler in the language

# Resources
I am using "[Writing a C Compiler](https://norasandler.com/2017/11/29/Write-a-Compiler.html)"
(blogposts by Nora Sandler) and "[An Incremental Approach to Compiler Construction](http://scheme2006.cs.uchicago.edu/11-ghuloum.pdf) "
(paper by Abdulaziz Ghuloum) as starting points for how to write a lexer, parser, and compiler.

# To Do

## ~~24~~ 21 Steps
 - [ ] **Integers**: ability to return a single integer
 - [ ] **Immediate Constants**: constants and types
 - [ ] **Unary Primatives**: primatives such as `not`, `decrement`, and `bit shift`
 - [ ] **Binary Primatives**: primatives such as `addition`,  `less than` and `modulo`. Start using the stack
 - [ ] **Local Variables**: local vars using the stack
 - [ ] **Conditional Expressions**: if statements
 - [ ] **Heap Allocation**: more complex data types (ie strings) and other heap objects
 - [ ] **Procedure Calls**: calling functions (stating with anonymous lambdas)
 - [ ] **Closures**: defining functions as Closures
 - [ ] **Tail Calls**: recursive functions
 - [ ] **Complex Constants**: maybe like basic structs? not exactly sure
 - [ ] **Assignment**: mutable Variables
 - [ ] **Extend Syntax**: loosen up the syntax, add more forms
 - [ ] **Symbols, Libraries, and Seperate Compilation**:  Precompiled libs, not sure about the other stuff
 - [ ] **Foreign Functions**: functions from the OS like `exit` and `write`
 - [ ] **Error Checking and Safe Primatives**: Errors and other debugging niceties
 - [ ] **Variable-arity Procedures**: No idea what this means.
 - [ ] **Apply**: Also no idea. Probably Scheme specific?
 - [ ] **Output Ports**: writing to output buffers
 - [ ] **Write and Display**: write without closures
 - [ ] **Input Ports**:  reading from input buffers



### Step 1: Return Integers
- [x] Lexer 
- [x] Parser 
- [x] Code Gen
- [x] Better comments
- [x] Better logging
- [x] Clean up code
- [ ] Test Cases
- [ ] Write-Up
- [ ] Github Release
