#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum InstructionType {
    PLUS,
    MINUS,
    RIGHT,
    LEFT,
    PUT_CHAR,
    READ_CHAR,
    JMP_IF_ZERO,
    JMP_IF_NOT_ZERO,
    INVALID,
}

impl InstructionType {
    pub fn from_char(c: char) -> Self {
        match c {
            '+' => InstructionType::PLUS,
            '-' => InstructionType::MINUS,
            '>' => InstructionType::RIGHT,
            '<' => InstructionType::LEFT,
            '.' => InstructionType::PUT_CHAR,
            ',' => InstructionType::READ_CHAR,
            '[' => InstructionType::JMP_IF_ZERO,
            ']' => InstructionType::JMP_IF_NOT_ZERO,
            _ => InstructionType::INVALID,
        }
    }
    
    pub fn to_char(ins_type: InstructionType) -> char {
        match ins_type {
            InstructionType::PLUS => '+',
            InstructionType::MINUS => '-',
            InstructionType::RIGHT => '>',
            InstructionType::LEFT => '<',
            InstructionType::PUT_CHAR => '.',
            InstructionType::READ_CHAR => ',',
            InstructionType::JMP_IF_ZERO => '[',
            InstructionType::JMP_IF_NOT_ZERO => ']',
            InstructionType::INVALID => '0',
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub ins_type: InstructionType,
    pub argument: usize,
}

impl Instruction {
    pub fn new(ins_type: InstructionType, arg: usize) -> Self {
        Instruction{ins_type: ins_type, argument: arg}
    }
}