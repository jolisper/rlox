use crate::chunk::{Chunk, OpCode};
use crate::debug;
use crate::value::Value;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

// The C version of the VM is a global static variable,
// but static mut variables in Rust may lead to unsafe
// code, so a heap/stack allocated version is prefered
// instead.
pub struct VM {
    chunk: Option<Chunk>,
    // The C version has an instruction pointer (ip) that
    // points directly to some position in the chunk for
    // performance reasons.
    // Here and offset is prefered because deref raw pointer
    // in Rust is not a safe operation. The Chunk has a
    // complementary get method that receives the offset.
    offset: usize,
}

impl VM {
    fn new() -> VM {
        VM {
            chunk: None,
            offset: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        return self.run();
    }

    fn run(&mut self) -> InterpretResult {
        // Each turn through loop, read and execute a single bytecode instruction.
        loop {
            #[cfg(feature = "debug-trace-execution")]
            {
                debug::dissassemble_instruction(&self.chunk.as_ref().unwrap(), self.offset);
            }

            // The first byte of any instruction is the opcode.
            let opcode = self.read_byte();

            // Given a numeric opcode, we need to get to the right Rust code that
            // implements that instruction’s semantics. This process is called
            // “decoding” or “dispatching” the instruction.
            match opcode {
                OpCode::OpReturn => return InterpretResult::InterpretOk,
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    debug::print_value(constant);
                    println!();
                }
            }
        }
    }

    fn read_byte(&mut self) -> OpCode {
        let opcode = self
            .chunk
            .as_ref()
            .expect("The chunk is not present.")
            .get(self.offset);

        // The opcodes are 1 byte lenght, so...
        self.offset += 1;

        opcode
    }

    fn read_constant(&mut self) -> Value {
        let chunk = self.chunk.as_ref().expect("the chunk is not present.");
        let index = chunk.get(self.offset) as usize;

        // The constants index are 1 byte lenght, so...
        self.offset += 1;

        chunk.get_constant_value(index)
    }
}

pub fn init_vm() -> VM {
    return VM::new();
}

pub fn free_vm() {}
