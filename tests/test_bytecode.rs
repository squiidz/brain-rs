extern crate brain;

use brain::{Compiler, ByteCode};

#[test]
fn test_with_bytecode() {
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut comp = Compiler::new(code);
    let instruction = comp.compile();
    let by = ByteCode::generate_bytecode(instruction);
    let mut byte_code = ByteCode::new(&by);
    println!("{:?}", byte_code);
    byte_code.execute();
}
