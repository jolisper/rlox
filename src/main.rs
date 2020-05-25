mod chunk;
mod debug;
mod value;

use chunk::{add_constant, init_chunk, write_chunk, OpCode};
use debug::dissassemble_chunk;

fn main() {
    let mut c = init_chunk();

    let constant = add_constant(&mut c, 1.2);
    write_chunk(&mut c, OpCode::OpConstant as u8, 123);
    write_chunk(&mut c, constant as u8, 123);

    write_chunk(&mut c, OpCode::OpReturn as u8, 123);

    dissassemble_chunk(&mut c, "test chunk");
}
