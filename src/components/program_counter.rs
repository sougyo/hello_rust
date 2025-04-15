pub struct ProgramCounter {
    value: u16,
    prev_clock: bool,
}

impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter {
            value: 0,
            prev_clock: false,
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
}