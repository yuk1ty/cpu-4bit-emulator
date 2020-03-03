pub struct Register {
    pub register_a: u8, // register a
    pub register_b: u8, // register b
    pub carry_flag: u8, // carry flag
    pub pc: u8, // program counter
}

impl Register {

    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            carry_flag: 0,
            pc: 0
        }
    }

    pub fn pc(&self) -> u8 {
        self.pc
    }

    pub fn set_pc(&mut self, new_value: u8) {
        self.pc = new_value;
    }

    pub fn incr_pc(&mut self) {
        self.pc += 1;
    }

    pub fn carry_flag(&self) -> u8 {
        self.carry_flag
    }

    pub fn set_carry_flag(&mut self, new_value: u8) {
        self.carry_flag = new_value;
    }

    pub fn register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_a(&mut self, new_value: u8) {
        self.register_a = new_value;
    }

    pub fn register_b(&self) -> u8 {
        self.register_b
    }

    pub fn set_register_b(&mut self, new_value: u8) {
        self.register_b = new_value;
    }
}