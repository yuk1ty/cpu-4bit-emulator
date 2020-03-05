pub struct Rom {
    pub memory_array: Vec<u8>,
}

impl Rom {
    pub fn new(memory_array: Vec<u8>) -> Self {
        Self { memory_array }
    }

    pub fn read(&self, pc: u8) -> u8 {
        self.memory_array[pc as usize]
    }

    pub fn size(&self) -> u8 {
        self.memory_array.len() as u8
    }
}
