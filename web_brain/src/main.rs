#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate serde_derive;
extern crate brain;

use rocket_contrib::JSON;
use brain::prelude::*;

#[derive(Deserialize)]
struct Code {
    code: String,
    args: String,
}

#[post("/run", format = "application/json", data = "<code>")]
fn interpret(code: JSON<Code>) -> String {
    let data = &code.code;
    let read_buffer = code.args.clone();
    let mut write_buffer: Vec<u8> = Vec::new();

    execute(data, read_buffer.as_bytes(), &mut write_buffer);
    write_buffer.iter().map(|c| *c as char).collect::<String>()
}

fn main() {
    rocket::ignite().mount("/api", routes![interpret]).launch();
}