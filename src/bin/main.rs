use cpu_4bit_emulator::emulator::CpuEmulator;
use cpu_4bit_emulator::port::Port;
use cpu_4bit_emulator::register::Register;
use cpu_4bit_emulator::rom::Rom;

fn main() {
    let rom = Rom::new(vec![0b00110001, 0b00000001, 0b01000000, 0b10010000]);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => emulator.out(),
        Err(err) => eprintln!("{:?}", err),
    }
}
