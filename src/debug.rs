use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub fn dissassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count() {
        offset = dissassemble_instruction(chunk, offset);
    }
}

fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_line(offset))
    }
    match chunk.get(offset) {
        OpCode::OpConstant => return constant_instruction("OP_CONSTANT", &chunk, offset),
        OpCode::OpReturn => return simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{} ", name);
    return offset + 1;
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.get(offset + 1) as usize;
    print!("{:<16} {:4} '", name, constant_index);
    print_value(chunk.get_constant_value(constant_index));
    print!("'\n");
    return offset + 2;
}

fn print_value(value: Value) {
    print!("{:?}", value);
}
