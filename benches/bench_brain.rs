#![feature(test)]
extern crate test;
extern crate brain;

use brain::prelude::*;

#[bench]
fn bench_hello_world(b: &mut test::Bencher) {
    let read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    b.iter(move || {
        execute(code, read_buffer.as_bytes(), &mut write_buffer);
    });
}
