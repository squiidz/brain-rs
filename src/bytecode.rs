use std::io::{Read, Write};
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
    WRITE(usize),
    READ(usize),
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
            InstructionType::PUT_CHAR => ByteCodeType::WRITE(ins.argument),
            InstructionType::READ_CHAR => ByteCodeType::READ(ins.argument),
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
            "WRITE" => ByteCodeType::WRITE(code_arr[1].parse().unwrap()),
            "READ" => ByteCodeType::READ(code_arr[1].parse().unwrap()),
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
            ByteCodeType::WRITE(i) => write!(f, "WRITE {}", i),
            ByteCodeType::READ(i) => write!(f, "READ {}", i),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[allow(dead_code)]
pub struct ByteCode {
    byte_code: Vec<ByteCodeType>,
    memory: [usize; 30000],
    index: usize,
    length: usize,
}

impl ByteCode {
    pub fn new(code: &str) -> ByteCode {
        let bcs = code.lines().map(|bc| ByteCodeType::from(bc) ).collect::<Vec<ByteCodeType>>();
        //println!("BCS: {:?}", bcs);
        let length = bcs.len();
        //println!("BCS LENGTH: {:?}", length);
        ByteCode {
            byte_code: bcs,
            memory: [0; 30000],
            index: 0,
            length: length,
        }
    }

    pub fn generate_bytecode(insts: Vec<Instruction>) -> String {
        let mut byte_codes = String::new();
        let mut loop_stack: Vec<usize> = Vec::new();
        for (i, inst) in insts.iter().enumerate() {
            //println!("{:?}", inst);
            if inst.ins_type != InstructionType::NEW_LINE {
                let ins = match inst.ins_type {
                    InstructionType::JMP_IF_ZERO => {
                        loop_stack.push(i);
                        Instruction {
                            ins_type: InstructionType::JMP_IF_ZERO,
                            position: inst.position,
                            argument: loop_stack.len(),
                        }
                    },
                    InstructionType::JMP_IF_NOT_ZERO => {
                        Instruction {
                            ins_type: InstructionType::JMP_IF_NOT_ZERO,
                            position: inst.position,
                            argument: (loop_stack.pop().expect("Error generate bytecode")),
                        }
                    },
                    _ => inst.clone(),
                };
                byte_codes.push_str(&format!("{}", ByteCodeType::from(&ins).to_string()));
                if i < insts.len() - 1 {
                    byte_codes.push('\n');
                }
            }
        }
        byte_codes
    }

    pub fn execute<R: Read, W: Write>(&mut self, r: &mut R, w: &mut W) {
        let mut inst_point: usize = 0;

        //println!("CALL EXECUTE");
        while inst_point <= self.length - 1{
            let bc = &self.byte_code[inst_point];
            match *bc {
                ByteCodeType::IADD(v) => {
                    //println!("CALL ADD");
                    self.memory[self.index] += v;
                },
                ByteCodeType::ISUB(v) => {
                    //println!("CALL SUB");
                    self.memory[self.index] -= v;
                },
                ByteCodeType::LOOP(_) => {
                    //println!("CALL LOOP");
                },
                ByteCodeType::END(v) => {
                    //println!("CALL END: {}", v);
                    if self.memory[self.index] != 0 {
                        inst_point = v;
                    }
                },
                ByteCodeType::FOWARD(v) => {
                    //println!("CALL FOWARD: {}", v);
                    self.index += v;
                },
                ByteCodeType::BACKWARD(v) => {
                    //println!("CALL BACKWARD: {}", v);
                    self.index -= v;
                },
                ByteCodeType::WRITE(v) => {
                    //println!("CALL WRITE");
                    for _ in 0..v {
                        w.write(&[self.memory[self.index] as u8]);
                    }
                },
                ByteCodeType::READ(v) => {
                    for _ in 0..v {
                        r.read(&mut [self.memory[self.index] as u8]);
                    }
                },
                _ => continue,
            }
            inst_point += 1;
        }
    }
}
