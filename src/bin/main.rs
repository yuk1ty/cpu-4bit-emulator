use cpu_4bit_emulator::emulator::CpuEmulator;
use cpu_4bit_emulator::port::Port;
use cpu_4bit_emulator::register::Register;
use cpu_4bit_emulator::rom::Rom;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let mut f = BufReader::new(File::open("example/simple_calc.sasm").expect("file not found"));
    let ops = f.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let mut source = Vec::new();
    for op in ops {
        let v: Vec<&str> = op.split(' ').collect();
        for value in v {
            let cloned = value.to_string();
            source.push(cloned);
        }
    }

    let rom = Rom::new(vec![0b00110001, 0b00000001, 0b01000000, 0b10010000]);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => emulator.out(),
        Err(err) => eprintln!("{:?}", err),
    }
}
