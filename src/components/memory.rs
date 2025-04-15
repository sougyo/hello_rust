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