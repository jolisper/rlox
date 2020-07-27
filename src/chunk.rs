use crate::value::{init_value_array, write_value_array, Value, ValueArray};
use std::convert::From;

#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            byte if byte == OpCode::OpConstant as u8 => OpCode::OpConstant,
            byte if byte == OpCode::OpReturn as u8 => OpCode::OpReturn,
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
