use crate::value::{init_value_array, write_value_array, Value, ValueArray};
use std::convert::From;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBSTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            byte if byte == OpCode::OP_CONSTANT as u8 => OpCode::OP_CONSTANT,
            byte if byte == OpCode::OP_ADD as u8 => OpCode::OP_ADD,
            byte if byte == OpCode::OP_SUBSTRACT as u8 => OpCode::OP_SUBSTRACT,
            byte if byte == OpCode::OP_MULTIPLY as u8 => OpCode::OP_MULTIPLY,
            byte if byte == OpCode::OP_DIVIDE as u8 => OpCode::OP_DIVIDE,
            byte if byte == OpCode::OP_NEGATE as u8 => OpCode::OP_NEGATE,
            byte if byte == OpCode::OP_RETURN as u8 => OpCode::OP_RETURN,
            _ => panic!("Unkown opcode!"),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.code.len()
    }

    pub fn get(&self, offset: usize) -> OpCode {
        OpCode::from(self.code[offset])
    }

    pub fn get_line(&self, offset: usize) -> usize {
        *self.lines.get(offset).unwrap()
    }

    pub fn get_constant_value(&self, offset: usize) -> f64 {
        self.constants.get(offset)
    }
}

pub fn init_chunk() -> Chunk {
    Chunk {
        code: vec![],
        lines: vec![],
        constants: init_value_array(),
    }
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: usize) {
    chunk.code.push(byte);
    chunk.lines.push(line);
}

pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
    write_value_array(&mut chunk.constants, value);
    return chunk.constants.count() - 1;
}
