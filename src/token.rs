#[derive(Debug)]
pub enum Register {
    A,
    B,
}

impl From<String> for Register {
    fn from(a: String) -> Self {
        if a == *"A" {
            Register::A
        } else if a == *"B" {
            Register::B
        } else {
            panic!("couldn't parse")
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Mov(Register, u8),
    MovAB,
    MovBA,
    Add(Register, u8),
    Jmp(u8),
    Jnc(u8),
    In(Register),
    OutIm(u8),
    OutB,
}
