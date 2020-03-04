use crate::error::EmulatorErr;
use crate::op::Opcode;
use num_traits::FromPrimitive;

pub struct CpuEmulator;

type ImData = u8;

impl CpuEmulator {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, data: u8) -> Result<(Opcode, ImData), EmulatorErr> {
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
}

#[cfg(test)]
mod tests {
    use crate::emulator::CpuEmulator;
    use crate::op::Opcode;

    #[test]
    fn test_use_im_opcode() {
        let data = 0b00000011;
        let emu = CpuEmulator::new();
        let actual = emu.decode(data);
        let expected_im: u8 = 0b0011;
        assert_eq!(actual.unwrap(), (Opcode::AddA, expected_im));
    }

    #[test]
    fn test_not_use_im_opcode() {
        let data = 0b11100001;
        let emu = CpuEmulator::new();
        let actual = emu.decode(data);
        let expected_im: u8 = 0;
        assert_eq!(actual.unwrap(), (Opcode::Jnc, expected_im));
    }
}
