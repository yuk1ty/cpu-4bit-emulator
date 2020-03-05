use crate::error::EmulatorErr;
use crate::op::Opcode;
use crate::port::Port;
use crate::register::Register;
use crate::rom::Rom;
use num_traits::FromPrimitive;
use std::cell::RefCell;
use std::borrow::Borrow;

pub struct CpuEmulator {
    register: RefCell<Register>,
    port: RefCell<Port>,
    rom: RefCell<Rom>,
}

type ImData = u8;

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

    fn decode(&self, data: u8) -> Result<(Opcode, ImData), EmulatorErr> {
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
                | Opcode::OutIm => Ok((opcode, im)),
                Opcode::Jnc | Opcode::InA | Opcode::InB | Opcode::OutB => Ok((opcode, 0)),
            }
        } else {
            // never comes
            Err(EmulatorErr::new("No match for opcode"))
        }
    }

    pub fn proceed(&self) -> Result<(), EmulatorErr> {
        let data = self.fetch();
        let (opcode, im) = self.decode(data)?;

        match opcode {
            Opcode::MovA => Ok(self.mov_a(im)),
            Opcode::MovB => Ok(self.mov_b(im)),
            Opcode::AddA => Ok(self.add_a(im)),
            Opcode::AddB => Ok(self.add_b(im)),
            _ => unimplemented!(), // TODO
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
}

#[cfg(test)]
mod tests {
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
        let proceeded = emu.proceed();

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
        let proceeded = emu.proceed();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 0);
        assert_eq!(emu.register.borrow().register_b(), 1);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }

    #[test]
    fn test_add_a_without_carrying() {
        let rom = Rom::new(vec![0b00000001]);
        let mut register = Register::new();
        register.set_register_a(1);
        let port = Port::new(0b0000, 0b0000);
        let emu = CpuEmulator::with(register, port, rom);
        let proceeded = emu.proceed();

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
        let proceeded = emu.proceed();

        assert!(proceeded.is_ok());
        assert_eq!(emu.register.borrow().register_a(), 0);
        assert_eq!(emu.register.borrow().register_b(), 2);
        assert_eq!(emu.register.borrow().pc(), 1);
        assert_eq!(emu.register.borrow().carry_flag(), 0);
    }
}
