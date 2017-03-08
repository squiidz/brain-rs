extern crate brain;

use brain::{compiler, machine};
use std::io::{self, Read};
use std::fs::File;
use std::env;

const ERROR: &'static str = "[Error]";

fn main() {
    let inp = io::stdin();
    let out = io::stdout();
    let args = env::args().collect::<Vec<String>>();
    let file_name = match args.last() {
        Some(name) => name,
        None => return,
    };
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("{} Source file needed.", ERROR);
            return
        },
    };

    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => { },
        Err(_) => {
            println!("{} Source file not found.", ERROR);
            return
        },
    }

    let mut cmp = compiler::Compiler::new(&code);
    let instructions = cmp.compile();
    let mut machine = machine::Machine::new(instructions, inp, out);
    machine.execute();
}
