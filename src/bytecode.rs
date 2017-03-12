use std::fmt::{self, Display};
use instruction::{Instruction, InstructionType};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ByteCodeType {
    ICONST(i32),
    IADD(usize),
    ISUB(usize),
    FOWARD(usize),
    BACKWARD(usize),
    LOOP(usize),
    END(usize),
    WRITE,
    READ,
    INVALID, 
}

impl<'a> From<&'a Instruction> for ByteCodeType {
    fn from(ins: &Instruction) -> Self {
        match ins.ins_type {
            InstructionType::PLUS => ByteCodeType::IADD(ins.argument),
            InstructionType::MINUS => ByteCodeType::ISUB(ins.argument),
            InstructionType::RIGHT => ByteCodeType::FOWARD(ins.argument),
            InstructionType::LEFT => ByteCodeType::BACKWARD(ins.argument),
            InstructionType::JMP_IF_ZERO => ByteCodeType::LOOP(ins.argument),
            InstructionType::JMP_IF_NOT_ZERO => ByteCodeType::END(ins.argument),
            InstructionType::PUT_CHAR => ByteCodeType::WRITE,
            InstructionType::READ_CHAR => ByteCodeType::READ,
            _ => { ByteCodeType::INVALID },
        }
    }
}

impl<'a> From<&'a str> for ByteCodeType {
    fn from(line: &str) -> ByteCodeType {
        let code_arr = line.split_whitespace().collect::<Vec<&str>>();
        match code_arr[0] {
            "IADD" => ByteCodeType::IADD(code_arr[1].parse().unwrap()),
            "ISUB" => ByteCodeType::ISUB(code_arr[1].parse().unwrap()),
            "FOWARD" => ByteCodeType::FOWARD(code_arr[1].parse().unwrap()),
            "BACKWARD" => ByteCodeType::BACKWARD(code_arr[1].parse().unwrap()),
            "LOOP" => ByteCodeType::LOOP(code_arr[1].parse().unwrap()),
            "END" => ByteCodeType::END(code_arr[1].parse().unwrap()),
            "WRITE" => ByteCodeType::WRITE,
            "READ" => ByteCodeType::READ,
            _ => ByteCodeType::INVALID,
        }
    }
}

impl Display for ByteCodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ByteCodeType::IADD(i) => write!(f, "IADD {}", i),
            ByteCodeType::ISUB(i) => write!(f, "ISUB {}", i),
            ByteCodeType::FOWARD(i) => write!(f, "FOWARD {}", i),
            ByteCodeType::BACKWARD(i) => write!(f, "BACKWARD {}", i),
            ByteCodeType::LOOP(i) => write!(f, "LOOP {}", i),
            ByteCodeType::END(i) => write!(f, "END {}", i),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct ByteCode {
    byte_code: Vec<ByteCodeType>,
    memory: Vec<usize>,
    index: usize,
    length: usize,
}

impl ByteCode {
    pub fn new(code: &str) -> ByteCode {
        let bcs = code.lines().map(|bc| ByteCodeType::from(bc) ).collect::<Vec<ByteCodeType>>();
        let length = bcs.len();
        ByteCode {
            byte_code: bcs,
            memory: Vec::new(),
            index: 0,
            length: length,
        }
    }

    pub fn generate_bytecode(insts: &[Instruction]) -> String {
        insts.iter()
        .filter(|inst| inst.ins_type != InstructionType::NEW_LINE)
        .map(|inst| format!("{}\n", ByteCodeType::from(inst).to_string()))
        .collect::<String>()
    }

    pub fn execute(&mut self) {
        for (i, bc) in self.byte_code.iter().enumerate() {
            match *bc {
                ByteCodeType::IADD(v) => {
                    if self.memory.len() <= self.index {
                        self.memory.push(v);
                        continue
                    }
                    self.memory[self.index] += v;
                },
                _ => return,
            }
        }
    }
}