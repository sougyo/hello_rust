pub struct ProgramCounter {
    value: u16,
    prev_clock: bool,
    memory: Vec<u8>, // メモリを追加
}

impl ProgramCounter {
    pub fn new(memory_size: usize) -> Self {
        ProgramCounter {
            value: 0,
            prev_clock: false,
            memory: vec![0; memory_size], // 指定されたサイズのメモリを初期化
        }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, address: u16) {
        self.value = address;
    }

    pub fn update(&mut self, clock: bool) {
        if clock && !self.prev_clock {
            self.value = self.value.wrapping_add(1);
        }
        self.prev_clock = clock;
    }

    pub fn load_memory(&mut self, address: u16, data: u8) {
        if (address as usize) < self.memory.len() {
            self.memory[address as usize] = data;
        }
    }

    pub fn read(&self) -> u8 {
        self.memory[self.value as usize]
    }
}