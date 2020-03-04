pub struct Register {
    pub register_a: u8, // register a
    pub register_b: u8, // register b
    pub carry_flag: u8, // carry flag
    pub pc: u8,         // program counter
}

impl Register {
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            carry_flag: 0,
            pc: 0,
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

#[cfg(test)]
mod tests {
    use crate::register::Register;

    #[test]
    fn test_set_register_a() {
        let mut register = Register::new();
        register.set_register_a(0b0110);
        assert_eq!(register.register_a(), 0b0110);
    }

    #[test]
    fn test_set_register_b() {
        let mut register = Register::new();
        register.set_register_b(0b0110);
        assert_eq!(register.register_b(), 0b0110);
    }

    #[test]
    fn test_set_carry_flag() {
        let mut register = Register::new();
        register.set_carry_flag(1);
        assert_eq!(register.carry_flag(), 1);
    }

    #[test]
    fn test_set_pc() {
        let mut register = Register::new();
        register.set_pc(10);
        assert_eq!(register.pc(), 10);
    }

    #[test]
    fn test_incr_pc() {
        let mut register = Register::new();
        assert_eq!(register.pc(), 0);
        register.incr_pc();
        assert_eq!(register.pc(), 1);
        register.incr_pc();
        assert_eq!(register.pc(), 2);
    }
}
