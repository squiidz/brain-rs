extern crate brain;

use brain::{Compiler, ByteCode};

#[test]
fn test_with_bytecode() {
    let read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++[>+++++++++++<-]>.";
    let mut comp = Compiler::new(code);
    let instruction = comp.compile();
    let by = ByteCode::generate_bytecode(instruction);
    let mut byte_code = ByteCode::new(&by);
    byte_code.execute(&mut read_buffer.as_bytes(), &mut write_buffer);
    println!("{}", write_buffer.iter().map(|c| *c as char).collect::<String>());
}
