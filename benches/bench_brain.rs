#![feature(test)]
extern crate test;
extern crate brain;

use brain::prelude::*;

#[bench]
fn bench_brainfuck_hello_world(b: &mut test::Bencher) {
    let read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    b.iter(move || {
        execute(code, read_buffer.as_bytes(), &mut write_buffer);
    });
}

#[bench]
fn bench_bytecode_hello_world(b: &mut test::Bencher) {
    let mut read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut byte_code = ByteCode::new(&get_bytecode(code));

    b.iter(move || {
        byte_code.execute(&mut read_buffer.as_bytes(), &mut write_buffer);
    });
}

fn get_bytecode(code: &str) -> String {
    let mut cmp = Compiler::new(&code);
    let mut instructions = cmp.compile();
    ByteCode::generate_bytecode(instructions)
}
