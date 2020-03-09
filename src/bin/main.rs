use cpu_4bit_emulator::rom::Rom;
use cpu_4bit_emulator::register::Register;
use cpu_4bit_emulator::port::Port;
use cpu_4bit_emulator::emulator::CpuEmulator;

fn main() {
    let rom = Rom::new(vec![0b00110001, 0b00000001]);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => {
            let a = emulator.register().register_a();
            println!("register_a: {}", a);
        },
        Err(err) => eprintln!("{:?}", err)
    }
}
