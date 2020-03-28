use crate::error::EmulatorErr;
use crate::token::{Register, Token};

pub struct Parser {
    pos: usize,
    source: Vec<String>,
}

impl Parser {
    pub fn new(source: Vec<String>) -> Parser {
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
                let lhs = self.source.get(self.pos).unwrap();
                self.pos += 1;
                let rhs = self.source.get(self.pos).unwrap();

                let token = if lhs == "B" || rhs == "A" {
                    Token::MovBA
                } else if lhs == "A" || rhs == "B" {
                    Token::MovAB
                } else {
                    Token::Mov(Register::from(lhs.to_string()), rhs.to_string())
                };

                result.push(token);
            }

            if op == "add" {
                self.pos += 1;
                let lhs = self.source.get(self.pos).unwrap();
                self.pos += 1;
                let rhs = self.source.get(self.pos).unwrap();

                let token = Token::Add(Register::from(lhs.to_string()), rhs.to_string());

                result.push(token);
            }

            if op == "jmp" {
                self.pos += 1;
                let im = self.source.get(self.pos).unwrap();
                result.push(Token::Jmp(im.to_string()));
            }

            if op == "jnc" {
                self.pos += 1;
                let im = self.source.get(self.pos).unwrap();
                result.push(Token::Jnc(im.to_string()))
            }

            if op == "in" {
                self.pos += 1;
                let lhs = self.source.get(self.pos).unwrap();
                self.pos += 1;
                let im = self.source.get(self.pos).unwrap();
                result.push(Token::In(
                    Register::from(lhs.to_string()),
                    im.to_string(),
                ))
            }

            if op == "out" {
                self.pos += 1;
                let im = self.source.get(self.pos).unwrap();
                result.push(Token::OutIm(im.to_string()))
            }

            self.pos += 1;
        }

        Ok(result)
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
            "0b0001".to_string(),
            "add".to_string(),
            "A".to_string(),
            "0b0001".to_string(),
        ];
        let mut parser = Parser::new(code);
        let result = parser.parse().unwrap();
        assert_eq!(result.len(), 2);
    }
}
