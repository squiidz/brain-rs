extern crate brain;

use brain::prelude::*;

#[test]
fn test_with_string() {
    let read_buffer = String::new();
    let mut write_buffer: Vec<u8> = Vec::new();
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

   match execute(code, read_buffer.as_bytes(), &mut write_buffer) {
       Ok(_) => {
            println!("{:?}", write_buffer.iter().map(|c| *c as char).collect::<String>());
            println!("{:?}", read_buffer);
       },
       Err(e) => panic!(e),
   }
}
