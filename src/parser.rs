use crate::error::EmulatorErr;
use crate::token::{Register, Token};

pub struct Parser {
    pos: usize,
    source: Vec<String>,
}

impl Parser {
    pub fn new(operations: Vec<String>) -> Parser {
        let mut source = Vec::new();
        for operation in operations {
            let split: Vec<&str> = operation.split(' ').collect();
            for line in split {
                let cloned = line.to_string();
                source.push(cloned);
            }
        }

        Parser { pos: 0, source }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, EmulatorErr> {
        let mut result = Vec::new();

        loop {
            let op = self.source.get(self.pos);

            if op.is_none() {
                break;
            }

            let op = op.unwrap();

            if op == "mov" {
                self.pos += 1;
                let lhs = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse mov left hand side value"))?;

                self.pos += 1;
                let rhs = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse mov right hand side value"))?;

                let token = if lhs == "B" && rhs == "A" {
                    Token::MovBA
                } else if lhs == "A" && rhs == "B" {
                    Token::MovAB
                } else {
                    Token::Mov(
                        Register::from(lhs.to_string()),
                        self.from_binary_to_decimal(rhs)?,
                    )
                };

                result.push(token);
            }

            if op == "add" {
                self.pos += 1;
                let lhs = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse Add left hand side value"))?;

                self.pos += 1;
                let rhs = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse Add right hand side value"))?;

                let token = Token::Add(
                    Register::from(lhs.to_string()),
                    self.from_binary_to_decimal(rhs)?,
                );

                result.push(token);
            }

            if op == "jmp" {
                self.pos += 1;
                let im = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse jmp im value"))?;

                result.push(Token::Jmp(self.from_binary_to_decimal(im)?));
            }

            if op == "jnc" {
                self.pos += 1;
                let im = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse jnc im value"))?;

                result.push(Token::Jnc(self.from_binary_to_decimal(im)?));
            }

            if op == "in" {
                self.pos += 1;
                let lhs = self.source.get(self.pos).unwrap();
                result.push(Token::In(Register::from(lhs.to_string())));
            }

            if op == "out" {
                self.pos += 1;
                let im = self
                    .source
                    .get(self.pos)
                    .ok_or_else(|| EmulatorErr::new("Failed to parse out im value"))?;

                if im == "B" {
                    result.push(Token::OutB);
                } else {
                    result.push(Token::OutIm(self.from_binary_to_decimal(im)?));
                }
            }

            self.pos += 1;
        }

        Ok(result)
    }

    fn from_binary_to_decimal(&self, text: impl Into<String>) -> Result<u8, EmulatorErr> {
        let ret = text.into();
        let binary_to_decimal = u8::from_str_radix(&ret, 2);
        binary_to_decimal.map_err(|_| EmulatorErr::new(&format!("Failed to parse string: {}", ret)))
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::parser::Parser;

    #[test]
    fn parse_simple() {
        let code = vec![
            "mov".to_string(),
            "A".to_string(),
            "0001".to_string(),
            "add".to_string(),
            "A".to_string(),
            "0001".to_string(),
        ];
        let mut parser = Parser::new(code);
        let result = parser.parse().unwrap();
        assert_eq!(result.len(), 2);
    }
}
