use crate::error::EmulatorErr;
use crate::token::{Register, Token};

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Compiler
    }

    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, EmulatorErr> {
        unimplemented!()
    }
}
