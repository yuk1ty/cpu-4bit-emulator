pub struct Port {
    input: u8,
    output: u8,
}

impl Port {
    pub fn new() -> Self {
        Self {
            input: 0,
            output: 0,
        }
    }
}
