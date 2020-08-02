use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

#[allow(dead_code)]
pub fn dissassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count() {
        offset = dissassemble_instruction(chunk, offset);
    }
}

pub fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_line(offset))
    }
    fn opcode_name(opcode: OpCode) -> String {
        format!("{:?}", opcode)
    }
    match chunk.get(offset) {
        opcode @ OpCode::OpConstant => {
            return constant_instruction(opcode_name(opcode), &chunk, offset)
        }
        opcode @ OpCode::OpNegate | opcode @ OpCode::OpReturn => {
            return simple_instruction(opcode_name(opcode), offset)
        }
    }
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{} ", name);
    return offset + 1;
}

fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.get(offset + 1) as usize;
    print!("{:<16} {:4} '", name, constant_index);
    print_value(chunk.get_constant_value(constant_index));
    print!("'\n");
    return offset + 2;
}

pub fn print_value(value: Value) {
    print!("{:?}", value);
}
