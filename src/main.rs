mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use std::io::{self, BufRead, Read, Write};

fn main() {
    let mut vm = vm::init_vm();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(
            &args.get(1).expect(&format!("No argument ar index {}", 1)),
            &mut vm,
        );
    } else {
        eprintln!("Usage: rlox [path]\n");
        std::process::exit(64);
    }
}

#[allow(unused_must_use)]
fn repl(vm: &mut vm::VM) {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        print!("> ");
        std::io::stdout().flush();

        if !stdin.lock().read_line(&mut buffer).is_ok() {
            println!();
            break;
        }
        vm.interpret(&buffer);
    }
}

fn run_file(path: &str, vm: &mut vm::VM) {
    let source = read_file(path).expect(&format!("Error reading file at {}", path));
    let result = vm.interpret(&source);

    match result {
        vm::InterpretResult::InterpretCompileError => std::process::exit(65),
        vm::InterpretResult::InterpretRuntimeError => std::process::exit(70),
        _ => return,
    }
}

fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
