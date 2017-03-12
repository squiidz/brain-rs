#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate brain;

use rocket_contrib::JSON;
use rocket::response::NamedFile;
use brain::prelude::*;

use std::path::{PathBuf, Path};
use std::io;

#[derive(Deserialize)]
struct Code {
    code: String,
    args: String,
}

#[derive(Serialize)]
struct Output {
    output: String,
    length: usize,
    error: String,
}

#[post("/api/run", format = "application/json", data = "<code>")]
fn interpret(code: JSON<Code>) -> JSON<Output> {
    let data = &code.code;
    let read_buffer = code.args.clone();
    let mut write_buffer: Vec<u8> = Vec::new();

    let output = match execute(data, read_buffer.as_bytes(), &mut write_buffer) {
        Ok(_) => {
            Output {
                output: write_buffer.iter().map(|c| *c as char).collect::<String>(),
                length: write_buffer.len(),
                error: "".to_owned(),
            }
        },
        Err(e) => Output{
            output: "".to_owned(),
            length: 0,
            error: e.clone(),
        },
    };
    JSON(output)
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("dist/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, files, interpret]).launch();
}
