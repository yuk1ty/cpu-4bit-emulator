use crate::error::EmulatorErr;
use crate::op::Opcode;
use crate::port::Port;
use crate::register::Register;
use crate::rom::Rom;
use num_traits::FromPrimitive;
use std::cell::RefCell;

pub struct CpuEmulator {
    register: RefCell<Register>,
    port: RefCell<Port>,
    rom: RefCell<Rom>,
}

impl CpuEmulator {
    pub fn with(register: Register, port: Port, rom: Rom) -> Self {
        Self {
            register: RefCell::new(register),
            port: RefCell::new(port),
            rom: RefCell::new(rom),
        }
    }

    fn fetch(&self) -> u8 {
        let pc = self.register.borrow().pc();
        if self.rom.borrow().size() <= pc {
            return 0;
        }

        let code = self.rom.borrow().read(pc);

        self.register.borrow_mut().incr_pc();

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

    pub fn exec(&self) -> Result<(), EmulatorErr> {
        let data = self.fetch();
        let (opcode, im) = self.decode(data)?;

        match opcode {
            Opcode::MovA => Ok(self.mov_a(im)),
            Opcode::MovB => Ok(self.mov_b(im)),
            Opcode::AddA => Ok(self.add_a(im)),
            Opcode::AddB => Ok(self.add_b(im)),
            Opcode::MovA2B => Ok(self.mov_a2b()),
            Opcode::MovB2A => Ok(self.mov_b2a()),
            Opcode::Jmp => Ok(self.jmp(im)),
            Opcode::Jnc => Ok(self.jnc(im)),
            Opcode::InA => Ok(self.in_a()),
            Opcode::InB => Ok(self.in_b()),
            Opcode::OutB => Ok(self.out_b()),
            Opcode::OutIm => Ok(self.out_im(im)),
        }
    }

    fn mov_a(&self, im: u8) {
        self.register.borrow_mut().set_register_a(im);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn mov_b(&self, im: u8) {
        self.register.borrow_mut().set_register_b(im);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn mov_a2b(&self) {
        let register_b = self.register.borrow().register_b();
        self.register.borrow_mut().set_register_a(register_b);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn mov_b2a(&self) {
        let register_a = self.register.borrow().register_a();
        self.register.borrow_mut().set_register_b(register_a);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn add_a(&self, im: u8) {
        let existence = self.register.borrow().register_a();
        let new_value = existence + im;

        if new_value > 0x0f {
            self.register.borrow_mut().set_carry_flag(1);
        }

        self.register.borrow_mut().set_register_a(new_value & 0x0f);
    }

    fn add_b(&self, im: u8) {
        let existence = self.register.borrow().register_b();
        let new_value = existence + im;

        if new_value > 0x0f {
            self.register.borrow_mut().set_carry_flag(1);
        }

        self.register.borrow_mut().set_register_b(new_value & 0x0f)
    }

    fn jmp(&self, im: u8) {
        self.register.borrow_mut().set_pc(im);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn jnc(&self, im: u8) {
        if self.register.borrow().carry_flag() == 0 {
            self.register.borrow_mut().set_pc(im);
        }
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn in_a(&self) {
        let input_port = self.port.borrow().input();
        self.register.borrow_mut().set_register_a(input_port);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn in_b(&self) {
        let input_port = self.port.borrow().input();
        self.register.borrow_mut().set_register_b(input_port);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn out_b(&self) {
        let register_b = self.register.borrow().register_b();
        self.port.borrow_mut().set_output(register_b);
        self.register.borrow_mut().set_carry_flag(0);
    }

    fn out_im(&self, im: u8) {
        self.port.borrow_mut().set_output(im);
        self.register.borrow_mut().set_carry_flag(0);
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
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 1);
        assert_eq!(emu.register.borrow().register_b(), 0);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_mov_b() {
        let rom = Rom::new(vec![0b01110001]);
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 0);
        assert_eq!(emu.register.borrow().register_b(), 1);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_mov_a2b() {
        let rom = Rom::new(vec![0b00010000]);
        let mut register = Register::new();
        register.set_register_b(2);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        assert_eq!(emu.register.borrow().register_a(), 0);

        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 2);
        assert_eq!(emu.register.borrow().register_b(), 2);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_mov_b2a() {
        let rom = Rom::new(vec![0b01000000]);
        let mut register = Register::new();
        register.set_register_a(2);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        assert_eq!(emu.register.borrow().register_b(), 0);

        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 2);
        assert_eq!(emu.register.borrow().register_b(), 2);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_add_a_without_carrying() {
        let rom = Rom::new(vec![0b00000001]);
        let mut register = Register::new();
        register.set_register_a(1);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 2);
        assert_eq!(emu.register.borrow().register_b(), 0);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_add_b_without_carrying() {
        let rom = Rom::new(vec![0b01010001]);
        let mut register = Register::new();
        register.set_register_b(1);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 0);
        assert_eq!(emu.register.borrow().register_b(), 2);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_port_in_a() {
        let rom = Rom::new(vec![0b00100000]);
        let register = Register::new();
        assert_eq!(register.register_a(), 0b0000);
        let port = Port::new(0b0001, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_port_in_b() {
        let rom = Rom::new(vec![0b01100000]);
        let register = Register::new();
        assert_eq!(register.register_b(), 0b0000);
        let port = Port::new(0b0001, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.exec();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_b(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }
}

#[cfg(test)]
mod cpu_integration_tests {
    use crate::emulator::CpuEmulator;
    use crate::port::Port;
    use crate::register::Register;
    use crate::rom::Rom;

    #[test]
    fn test_mov_a_and_add_a() {
        let rom = Rom::new(vec![0b00110001, 0b00000001]);
        let rom_size = rom.size();
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        for _ in 0..rom_size {
            emu.exec().unwrap();
        }

        assert_eq!(emu.register.borrow().register_a(), 2);
        assert_eq!(emu.register.borrow().pc(), 2);
    }

    #[test]
    fn test_jmp() {
        // 0: MOV A, 0010
        // 1: ADD A, 0011
        // 2: JMP 0001
        let rom = Rom::new(vec![0b00110010, 0b00000011, 0b11110001]);
        let rom_size = rom.size();
        let register = Register::new();
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        for _ in 0..rom_size {
            emu.exec().unwrap();
        }

        assert_eq!(emu.register.borrow().pc(), 0b0001);
        assert_eq!(emu.register.borrow().carry_flag(), 0b0000);
        assert_eq!(emu.register.borrow().register_a(), 5);

        emu.exec().unwrap();
        assert_eq!(emu.register.borrow().register_a(), 8);
    }

    #[test]
    fn test_jnc_with_carry_flag_zero() {
        // 0: MOV A, 0010
        // 1: ADD A, 0011
        // 2: JNC 0001
        let rom = Rom::new(vec![0b00110010, 0b00000011, 0b11100001]);
        let rom_size = rom.size();
        let mut register = Register::new();
        register.set_carry_flag(0b0000);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        for _ in 0..rom_size {
            emu.exec().unwrap();
        }

        assert_eq!(emu.register.borrow().pc(), 0b00001);
        assert_eq!(emu.register.borrow().carry_flag(), 0b0000);
        assert_eq!(emu.register.borrow().register_a(), 5);

        emu.exec().unwrap();
        assert_eq!(emu.register.borrow().register_a(), 8);
    }

    #[test]
    fn test_jnc_with_carry_flag_on() {
        // 0: MOV A, 0010
        // 1: ADD A, 0011
        // 2: JNC 0001
        let rom = Rom::new(vec![0b00110010, 0b00000011, 0b11100001]);
        let rom_size = rom.size();
        let mut register = Register::new();
        register.set_carry_flag(1);
        assert_eq!(register.carry_flag(), 1);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);

        for _ in 0..rom_size {
            emu.exec().unwrap();
        }

        assert_eq!(emu.register.borrow().pc(), 0b0001);
        assert_eq!(emu.register.borrow().carry_flag(), 0b0000);
        assert_eq!(emu.register.borrow().register_a(), 5);

        emu.exec().unwrap();
        assert_eq!(emu.register.borrow().register_a(), 8);
    }
}
