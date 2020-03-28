use crate::error::EmulatorErr;
use crate::token::{Register, Token};

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Compiler
    }

    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, EmulatorErr> {
        let mut result = Vec::new();

        for token in tokens {
            match token {
                Token::Mov(Register::A, im) => {
                    result.push(self.gen_bin_code(0b0011, im)?);
                }
                Token::Mov(Register::B, im) => {
                    result.push(self.gen_bin_code(0b0111, im)?)
                }
                Token::MovAB => unimplemented!(),
                Token::MovBA => unimplemented!(),
                Token::Add(Register::A, im) => unimplemented!(),
                Token::Add(Register::B, im) => unimplemented!(),
                Token::Jmp(im) => unimplemented!(),
                Token::Jnc(im) => unimplemented!(),
                Token::In(Register::A, im) => unimplemented!(),
                Token::In(Register::B, im) => unimplemented!(),
                Token::OutB => unimplemented!(),
                Token::OutIm(im) => unimplemented!(),
            }
        }

        Ok(result)
    }

    fn gen_bin_code(&self, op: u8, im: String) -> Result<u8, EmulatorErr> {
        let shift_op = op << 4;
        let shift_data = im
            .parse::<u8>()
            .map_err(|_| EmulatorErr::new("Failed to parse im: {}"))?
            & 0x0f;
        Ok(shift_op | shift_data)
    }
}

#[cfg(test)]
mod compiler_tests {
    use crate::compiler::Compiler;
    use crate::token::Token::Mov;
    use crate::token::Register;

    #[test]
    fn test_compile_mov_a() {
        let compiler = Compiler::new();
        let program = compiler.compile(vec![Mov(Register::A, "1".to_string())]);
        assert_eq!(program.unwrap(), vec![0b00110001]);
    }

    #[test]
    fn test_compile_mov_b() {
        let compiler = Compiler::new();
        let program = compiler.compile(vec![Mov(Register::B, "1".to_string())]);
        assert_eq!(program.unwrap(), vec![0b01110001]);
    }
}
