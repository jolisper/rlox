mod chunk;
mod debug;
mod value;
mod vm;

use chunk::OpCode;

fn main() {
    let mut vm = vm::init_vm();
    let mut c = chunk::init_chunk();

    let constant = chunk::add_constant(&mut c, 1.2);
    chunk::write_chunk(&mut c, OpCode::OpConstant as u8, 123);
    chunk::write_chunk(&mut c, constant as u8, 123);
    chunk::write_chunk(&mut c, OpCode::OpNegate as u8, 123);

    chunk::write_chunk(&mut c, OpCode::OpReturn as u8, 123);

    vm.interpret(c);
}
