use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Opcode {
    AddA = 0b0000,
    AddB = 0b0101,
    MovA = 0b0011,
    MovB = 0b0111,
    MovA2B = 0b0001,
    MovB2A = 0b0100,
    Jmp = 0b1111,
    Jnc = 0b1110,
    InA = 0b0010,
    InB = 0b0110,
    OutB = 0b1001,
    OutIm = 0b1011,
}

