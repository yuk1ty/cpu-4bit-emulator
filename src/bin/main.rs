use cpu_4bit_emulator::compiler::Compiler;
use cpu_4bit_emulator::emulator::CpuEmulator;
use cpu_4bit_emulator::parser::Parser;
use cpu_4bit_emulator::port::Port;
use cpu_4bit_emulator::register::Register;
use cpu_4bit_emulator::rom::Rom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Invalid args. Usage: [command] [file_path]");
    }

    let f = BufReader::new(File::open(args.get(1).unwrap()).expect("file not found"));
    let ops = f.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let mut source = Vec::new();
    for op in ops {
        let v: Vec<&str> = op.split(' ').collect();
        for value in v {
            let cloned = value.to_string();
            source.push(cloned);
        }
    }

    let mut parser = Parser::new(source);
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{:?}", err),
    };

    let compiler = Compiler::new();
    let program = match compiler.compile(tokens) {
        Ok(program) => program,
        Err(err) => panic!("{:?}", err),
    };

    let rom = Rom::new(program);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    }
}
