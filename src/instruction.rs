#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum InstructionType {
    PLUS,
    MINUS,
    RIGHT,
    LEFT,
    PUT_CHAR,
    READ_CHAR,
    JMP_IF_ZERO,
    JMP_IF_NOT_ZERO,
    NEW_LINE,
    INVALID,
}

impl Into<char> for InstructionType {
    fn into(self) -> char {
        match self {
            InstructionType::PLUS => '+',
            InstructionType::MINUS => '-',
            InstructionType::RIGHT => '>',
            InstructionType::LEFT => '<',
            InstructionType::PUT_CHAR => '.',
            InstructionType::READ_CHAR => ',',
            InstructionType::JMP_IF_ZERO => '[',
            InstructionType::JMP_IF_NOT_ZERO => ']',
            InstructionType::NEW_LINE => '\n',
            InstructionType::INVALID => '0',
        }
    }
}

impl From<char> for InstructionType {
    fn from(c: char) -> Self {
        match c {
            '+' => InstructionType::PLUS,
            '-' => InstructionType::MINUS,
            '>' => InstructionType::RIGHT,
            '<' => InstructionType::LEFT,
            '.' => InstructionType::PUT_CHAR,
            ',' => InstructionType::READ_CHAR,
            '[' => InstructionType::JMP_IF_ZERO,
            ']' => InstructionType::JMP_IF_NOT_ZERO,
            '\n' | '\r' => InstructionType::NEW_LINE,
            _ => InstructionType::INVALID,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub ins_type: InstructionType,
    pub position: usize,
    pub argument: usize,
}

impl Instruction {
    pub fn new(ins_type: InstructionType, pos: usize, arg: usize) -> Self {
        Instruction{
            ins_type: ins_type,
            position: pos,
            argument: arg
        }
    }
}
