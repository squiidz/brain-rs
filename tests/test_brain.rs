extern crate brain;

use brain::{Compiler, Machine};

#[test]
fn test_with_string() {
    let mut read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    run_machine(code, read_buffer.as_mut_str(), write_buffer.as_mut_slice());

    println!("{:?}", write_buffer);
    println!("{:?}", read_buffer);
}

fn run_machine(code: &str, r: &mut str, w: &mut [u8]) {
    let mut comp = Compiler::new(code);
    let mut machine = Machine::new(comp.compile(), r.as_bytes(), w);
    machine.execute()
}