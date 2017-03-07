extern crate brain;

use brain::{compiler, machine};
use std::io::{self, Read};
use std::fs::File;
use std::env;

fn main() {
    let inp = io::stdin();
    let out = io::stdout();
    let file_name = env::args().collect::<Vec<String>>();
    let mut file = match File::open(file_name.last().unwrap()) {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    
    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => { },
        Err(e) => panic!(e),
    }
    
    let mut cmp = compiler::Compiler::new(&code);
    let instructions = cmp.compile();
    let mut machine = machine::Machine::new(instructions, inp, out);
    machine.execute();
}