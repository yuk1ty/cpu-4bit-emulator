use crate::error::EmulatorErr;
use crate::op::Opcode;
use crate::port::Port;
use crate::register::Register;
use crate::rom::Rom;
use num_traits::FromPrimitive;

pub struct CpuEmulator {
    register: Register,
    port: Port,
    rom: Rom,
}

impl CpuEmulator {
    pub fn with(register: Register, port: Port, rom: Rom) -> Self {
        assert!(
            rom.size() <= 16,
            "Maximum memory size is 16. This program can't work."
        );
        Self {
            register,
            port,
            rom,
        }
    }

    fn fetch(&self) -> u8 {
        let pc = self.register.pc();
        if self.rom.size() <= pc {
            return 0;
        }

        let code = self.rom.read(pc);

        code
    }

    fn decode(&self, data: u8) -> Result<(Opcode, u8), EmulatorErr> {
        let op = data >> 4;
        let im = data & 0x0f;

        if let Some(opcode) = FromPrimitive::from_u8(op) {
            match opcode {
                Opcode::AddA
                | Opcode::AddB
                | Opcode::MovA
                | Opcode::MovB
                | Opcode::MovA2B
                | Opcode::MovB2A
                | Opcode::Jmp
                | Opcode::Jnc
                | Opcode::OutIm => Ok((opcode, im)),
                Opcode::InA | Opcode::InB | Opcode::OutB => Ok((opcode, 0)),
            }
        } else {
            // never comes
            Err(EmulatorErr::new("No match for opcode"))
        }
    }

    pub fn exec(&mut self) -> Result<(), EmulatorErr> {
        loop {
            let data = self.fetch();
            let (opcode, im) = self.decode(data)?;

            match opcode {
                Opcode::MovA => self.mov_a(im),
                Opcode::MovB => self.mov_b(im),
                Opcode::AddA => self.add_a(im),
                Opcode::AddB => self.add_b(im),
                Opcode::MovA2B => self.mov_a2b(),
                Opcode::MovB2A => self.mov_b2a(),
                Opcode::Jmp => self.jmp(im),
                Opcode::Jnc => self.jnc(im),
                Opcode::InA => self.in_a(),
                Opcode::InB => self.in_b(),
                Opcode::OutB => self.out_b(),
                Opcode::OutIm => self.out_im(im),
            };

            // To prevent infinite loop
            if opcode != Opcode::Jmp && opcode != Opcode::Jnc {
                self.register.incr_pc();
            }

            if self.does_halt() {
                return Ok(());
            }
        }
    }

    fn does_halt(&self) -> bool {
        self.register.pc() >= self.rom.size()
    }

    fn mov_a(&mut self, im: u8) {
        self.register.set_register_a(im);
        self.register.set_carry_flag(0);
    }

    fn mov_b(&mut self, im: u8) {
        self.register.set_register_b(im);
        self.register.set_carry_flag(0);
    }

    fn mov_a2b(&mut self) {
        let register_b = self.register.register_b();
        self.register.set_register_a(register_b);
        self.register.set_carry_flag(0);
    }

    fn mov_b2a(&mut self) {
        let register_a = self.register.register_a();
        self.register.set_register_b(register_a);
        self.register.set_carry_flag(0);
    }

    fn add_a(&mut self, im: u8) {
        let existence = self.register.register_a();
        let new_value = existence + im;

        if new_value > 0x0f {
            self.register.set_carry_flag(1);
        }

        self.register.set_register_a(new_value & 0x0f);
    }

    fn add_b(&mut self, im: u8) {
        let existence = self.register.register_b();
        let new_value = existence + im;

        if new_value > 0x0f {
            self.register.set_carry_flag(1);
        }

        self.register.set_register_b(new_value & 0x0f)
    }

    fn jmp(&mut self, im: u8) {
        self.register.set_pc(im);
        self.register.set_carry_flag(0);
    }

    fn jnc(&mut self, im: u8) {
        if self.register.carry_flag() == 0 {
            self.register.set_pc(im);
        }
        self.register.set_carry_flag(0);
    }

    fn in_a(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_a(input_port);
        self.register.set_carry_flag(0);
    }

    fn in_b(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_b(input_port);
        self.register.set_carry_flag(0);
    }

    fn out_b(&mut self) {
        let register_b = self.register.register_b();
        self.port.set_output(register_b);
        self.register.set_carry_flag(0);
        println!("Port (B) Out: {}", self.port.output());
    }

    fn out_im(&mut self, im: u8) {
        self.port.set_output(im);
        self.register.set_carry_flag(0);
        println!("Port Out: {}", self.port.output());
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::emulator::CpuEmulator;
    use crate::port::Port;
    use crate::register::Register;
    use crate::rom::Rom;

    #[test]
    fn test_mov_a() {
        let rom = Rom::new(vec![0b00110001]);
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 1);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_b() {
        let rom = Rom::new(vec![0b01110001]);
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 1);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_a2b() {
        let rom = Rom::new(vec![0b00010000]);
        let mut register = Register::new();
        register.set_register_b(2);
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);

        assert_eq!(emu.register.register_a(), 0);

        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 2);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_mov_b2a() {
        let rom = Rom::new(vec![0b01000000]);
        let mut register = Register::new();
        register.set_register_a(2);
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);

        assert_eq!(emu.register.register_b(), 0);

        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 2);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_add_a_without_carrying() {
        let rom = Rom::new(vec![0b00000001]);
        let mut register = Register::new();
        register.set_register_a(1);
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 2);
        assert_eq!(emu.register.register_b(), 0);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_add_b_without_carrying() {
        let rom = Rom::new(vec![0b01010001]);
        let mut register = Register::new();
        register.set_register_b(1);
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 2);
        assert_eq!(emu.register.pc(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_jmp() {
        let rom = Rom::new(vec![0b11110010, 0b00110001, 0b01110010]);
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.pc(), 3);
        assert_eq!(emu.register.register_a(), 0);
        assert_eq!(emu.register.register_b(), 2);
    }

    #[test]
    fn test_port_in_a() {
        let rom = Rom::new(vec![0b00100000]);
        let register = Register::new();
        assert_eq!(register.register_a(), 0b0000);
        let port = Port::new(0b0001, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_a(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_port_in_b() {
        let rom = Rom::new(vec![0b01100000]);
        let register = Register::new();
        assert_eq!(register.register_b(), 0b0000);
        let port = Port::new(0b0001, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.register_b(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_port_out_b() {
        let rom = Rom::new(vec![0b10010000]);
        let mut register = Register::new();
        register.set_register_b(0b0001);
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.port.output(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }

    #[test]
    fn test_port_out_im() {
        let rom = Rom::new(vec![0b10110001]);
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let mut emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.port.output(), 1);
        assert_eq!(emu.register.carry_flag(), 0);
    }
}
