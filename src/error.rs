#[derive(Debug)]
pub struct EmulatorErr {
    msg: String,
}

impl EmulatorErr {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
