use super::memory::DFlipFlop;
use super::gates::{and_gate, or_gate, xor_gate};
use super::adders::four_bit_adder;

pub struct Register4Bit {
    flip_flops: [DFlipFlop; 4],
}

impl Register4Bit {
    pub fn new() -> Self {
        Register4Bit {
            flip_flops: [
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
            ],
        }
    }

    pub fn load(&mut self, data: [bool; 4], clock: bool) {
        for i in 0..4 {
            self.flip_flops[i].update(data[i], clock);
        }
    }

    pub fn output(&self) -> [bool; 4] {
        let mut result = [false; 4];
        for i in 0..4 {
            result[i] = self.flip_flops[i].output();
        }
        result
    }
}

pub struct ALU4Bit {
    reg_a: Register4Bit,
    reg_b: Register4Bit,
    result_reg: Register4Bit,
}

impl ALU4Bit {
    pub fn new() -> Self {
        ALU4Bit {
            reg_a: Register4Bit::new(),
            reg_b: Register4Bit::new(),
            result_reg: Register4Bit::new(),
        }
    }

    pub fn load_a(&mut self, data: [bool; 4], clock: bool) {
        self.reg_a.load(data, clock);
    }

    pub fn load_b(&mut self, data: [bool; 4], clock: bool) {
        self.reg_b.load(data, clock);
    }

    pub fn execute(&mut self, op_code: u8, clock: bool) {
        let a = self.reg_a.output();
        let b = self.reg_b.output();
        let mut result = [false; 4];

        match op_code {
            0 => {
                for i in 0..4 {
                    result[i] = and_gate(a[i], b[i]);
                }
            }
            1 => {
                for i in 0..4 {
                    result[i] = or_gate(a[i], b[i]);
                }
            }
            2 => {
                for i in 0..4 {
                    result[i] = xor_gate(a[i], b[i]);
                }
            }
            3 => {
                let (sum, _) = four_bit_adder(a, b);
                result = sum;
            }
            _ => {}
        }

        self.result_reg.load(result, clock);
    }

    pub fn get_result(&self) -> [bool; 4] {
        self.result_reg.output()
    }

    pub fn get_reg_a(&self) -> [bool; 4] {
        self.reg_a.output()
    }

    pub fn get_reg_b(&self) -> [bool; 4] {
        self.reg_b.output()
    }
}