pub struct Port {
    input: u8,
    output: u8,
}

impl Port {
    pub fn new(input: u8, output: u8) -> Self {
        Self {
            input: 0,
            output: 0,
        }
    }

    pub fn input(&self) -> u8 {
        self.input
    }

    pub fn output(&self) -> u8 {
        self.output
    }
}
