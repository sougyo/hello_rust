pub struct DFlipFlop {
    state: bool,
    prev_clock: bool,
}

impl DFlipFlop {
    pub fn new() -> Self {
        DFlipFlop {
            state: false,
            prev_clock: false,
        }
    }

    pub fn update(&mut self, data: bool, clock: bool) {
        if clock && !self.prev_clock {
            self.state = data;
        }
        self.prev_clock = clock;
    }

    pub fn output(&self) -> bool {
        self.state
    }
}

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size], // 指定されたサイズのメモリを初期化
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if (address as usize) < self.data.len() {
            self.data[address as usize]
        } else {
            0
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if (address as usize) < self.data.len() {
            self.data[address as usize] = value;
        }
    }
}
