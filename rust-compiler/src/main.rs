use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    //TODO: write command line argument parsing function once you have better rust knowledge
    let path = Path::new("main.md");
    let display = path.display();

    //open file
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    //extract contents as string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

}


fn lexer(s: String) -> String {
    s.matches()
}

//TODO: Testing
//TODO: Lexer
//TODO: Parser
