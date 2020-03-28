use crate::token::{Token, Register};
use crate::error::EmulatorErr;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Compiler
    }

    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, EmulatorErr> {
        unimplemented!()
    }
}