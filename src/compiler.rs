use instruction::{Instruction, InstructionType};

#[derive(Debug)]
pub struct Compiler {
    code: String,
    code_length: usize,
    position: usize,
    instructions: Vec<Instruction>
}

impl Compiler {
    pub fn new(code: &str) -> Self {
        Compiler {
            code: code.to_owned(),
            code_length: code.len(),
            position: 0,
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> &Vec<Instruction> {
        let mut loop_stack: Vec<usize> = Vec::new();

        while self.position < self.code_length {
            let current = self.code.chars().nth(self.position).unwrap();
            match current {
                '[' => loop_stack.push(self.emit_with_arg(InstructionType::JMP_IF_ZERO, 0)),
                ']' => {
                    let op_ins = loop_stack[loop_stack.len() - 1];
                    loop_stack = loop_stack[..loop_stack.len() - 1].to_vec();
                    let close_ins_pos = self.emit_with_arg(InstructionType::JMP_IF_NOT_ZERO, op_ins);
                    self.instructions[op_ins].argument = close_ins_pos;
                },
                _ => self.compile_foldable_instruction(current, InstructionType::from_char(current)),
            }
            self.position += 1;
        }
        &self.instructions
    }

    fn compile_foldable_instruction(&mut self, c: char, ins_type: InstructionType) {
        let mut count = 1;
        while self.position < self.code_length - 1 && self.code.chars().nth(self.position + 1).unwrap() == c {
            count += 1;
            self.position += 1;
        }
        self.emit_with_arg(ins_type, count);
    }

    fn emit_with_arg(&mut self, ins_type: InstructionType, arg: usize) -> usize {
        self.instructions.push(Instruction::new(ins_type, arg));
        self.instructions.len() - 1
    }
}
