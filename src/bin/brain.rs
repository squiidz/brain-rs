extern crate clap;
extern crate brain;

use clap::{App, Arg};
use brain::{Compiler, Machine, ByteCode};

use std::io::{self, Read};
use std::fs::File;

const ERROR: &'static str = "[Error]";

fn main() {
    let mut inp = io::stdin();
    let mut out = io::stdout();

    let matches = App::new("brain")
        .arg(Arg::with_name("file")
            .takes_value(true)
            .index(1)
            .required(true)
        ).arg(Arg::with_name("generate_bytecode")
            .short("g")
            .long("generate")
        ).arg(Arg::with_name("bytecode")
              .short("b")
              .long("bytecode")
              .conflicts_with("generate_bytecode")
        ).get_matches();


    let file_name = match matches.value_of("file") {
        Some(v) => v,
        None => {
            println!("{} Source file needed.", ERROR);
            return
        },
    };

    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("{} Source file not found.", ERROR);
            return
        },
    };

    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => { },
        Err(_) => {
            println!("{} Source invalid.", ERROR);
            return
        },
    }

    if matches.is_present("generate_bytecode") {
        let mut cmp = Compiler::new(&code);
        let instructions = cmp.compile();
        //println!("{:?}", instructions);
        let bytecode = ByteCode::generate_bytecode(instructions);
        print!("{}", bytecode);
    } else if matches.is_present("bytecode") {
        let mut bc_machine = ByteCode::new(&code);
        bc_machine.execute(&mut inp, &mut out);
    } else {
        let mut cmp = Compiler::new(&code);
        let instructions = cmp.compile();
        let mut machine = Machine::new(&instructions, inp, out);
        match machine.execute() {
            Ok(_) => {},
            Err(e) => {
                println!("{} {}", ERROR, e);
                return
            }
        }
    }
}
