pub enum Register {
    A,
    B,
}

impl From<String> for Register {
    fn from(a: String) -> Self {
        if a == "A".to_string() {
            Register::A
        } else if a == "B".to_string() {
            Register::B
        } else {
            panic!("couldn't parse")
        }
    }
}

pub enum Token {
    Mov(Register, String),
    MovAB,
    MovBA,
    Add(Register, String),
    Jmp(String),
    Jnc(String),
    In(Register, String),
    OutIm(String),
    OutB,
}
