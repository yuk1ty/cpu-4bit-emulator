pub struct Rom {
    pub memory_array: Vec<u8>,
}

impl Rom {
    const MEMORY_SIZE: usize = 16;

    pub fn new() -> Self {
        let memory_array = Vec::with_capacity(Self::MEMORY_SIZE);
        Self { memory_array }
    }

    pub fn read(&self, pc: u8) -> u8 {
        self.memory_array[pc as usize]
    }

    pub fn size(&self) -> u8 {
        self.memory_array.len() as u8
    }
}
