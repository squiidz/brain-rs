use std::io::{self, Read, Write};
use std::fmt::{self, Display};
use instruction::{Instruction, InstructionType};

pub struct Machine<'a, R: Read, W: Write> {
    code: &'a Vec<Instruction>,
    ip: usize,
    memory: [usize; 30000],
    dp: usize,
    input: R,
    output: W,
    read_buf: [u8; 1],
}

impl<'a, R: Read, W: Write> Display for Machine<'a, R, W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Machine ( code: {:?}, dp: {:?}, buf: {:?} )",
            self.code, self.dp, self.read_buf
        )
    }
}

impl<'a, R: Read, W: Write> Machine<'a, R, W> {
    pub fn new(ins: &'a Vec<Instruction>, inp: R, out: W) -> Self {
        Machine {
            code: ins,
            ip: 0,
            memory: [0; 30000],
            dp: 0,
            input: inp,
            output: out,
            read_buf: [0; 1],
        }
    }

    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            let ins = &self.code[self.ip];

            match ins.ins_type {
                InstructionType::PLUS => self.memory[self.dp] += ins.argument,
                InstructionType::MINUS => self.memory[self.dp] -= ins.argument,
                InstructionType::RIGHT => self.dp += ins.argument,
                InstructionType::LEFT => self.dp -= ins.argument,
                InstructionType::PUT_CHAR => {
                    for _ in 0..ins.argument {
                        self.put_char();
                    }
                },
                InstructionType::READ_CHAR => {
                    for _ in 0..ins.argument {
                        self.read_char();
                    }
                },
                InstructionType::JMP_IF_ZERO => {
                    if self.memory[self.dp] == 0 {
                        self.ip = ins.argument as usize;
                        continue
                    }
                },
                InstructionType::JMP_IF_NOT_ZERO => {
                    if self.memory[self.dp] != 0 {
                        self.ip = ins.argument as usize;
                        continue
                    }
                },
                InstructionType::NEW_LINE => {
                    self.ip += 1;
                    continue
                },
                _ => break,
            }
            self.ip += 1;
        }
    }

    fn read_char(&mut self) {
        self.input.read(&mut self.read_buf);
        self.memory[self.dp] = *self.read_buf.first().unwrap() as usize;

        println!("memory: {:?}", self.memory[self.dp]);
    }

    fn put_char(&mut self) -> io::Result<usize> {
        self.read_buf[0] = self.memory[self.dp] as u8;
        self.output.write(&[self.read_buf.last().unwrap().to_owned()])
    }
}
