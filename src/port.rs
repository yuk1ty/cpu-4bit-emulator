pub struct Port {
    input: u8,
    output: u8,
}

impl Port {
    pub fn new(input: u8, output: u8) -> Self {
        Self {
            input,
            output,
        }
    }

    pub fn input(&self) -> u8 {
        self.input
    }

    pub fn output(&self) -> u8 {
        self.output
    }

    pub fn set_output(&mut self, im: u8) {
        self.output = im;
    }
}
