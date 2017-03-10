extern crate brain;

use brain::prelude::*;

#[test]
fn test_with_string() {
    let mut read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    execute(code, read_buffer.as_bytes(), &mut write_buffer);

    println!("{:?}", write_buffer.iter().map(|c| *c as char).collect::<String>());
    println!("{:?}", read_buffer);
}
