use crate::components::gates::bool_array_to_u8;

use super::adders::{full_adder, four_bit_adder};
use super::gates::{and_gate, or_gate, u8_to_bool_array, xor_gate};
use super::memory::DFlipFlop;
use super::program_counter::ProgramCounter;

// 操作コードの定数
pub const OP_AND: u8 = 0;
pub const OP_OR: u8 = 1;
pub const OP_XOR: u8 = 2;
pub const OP_ADD: u8 = 3;

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
            OP_AND => {
                for i in 0..4 {
                    result[i] = and_gate(a[i], b[i]);
                }
            }
            OP_OR => {
                for i in 0..4 {
                    result[i] = or_gate(a[i], b[i]);
                }
            }
            OP_XOR => {
                for i in 0..4 {
                    result[i] = xor_gate(a[i], b[i]);
                }
            }
            OP_ADD => {
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


#[test]
fn test_program_counter_with_alu() {
    let mut pc = ProgramCounter::new(256); // メモリサイズを指定
    let mut alu = ALU4Bit::new();

    // メモリに操作コードをロード
    pc.load_memory(0, OP_AND); // AND操作
    pc.load_memory(1, OP_OR);  // OR操作
    pc.load_memory(2, OP_XOR); // XOR操作
    pc.load_memory(3, OP_ADD); // 加算操作

    // テストデータ
    let a_val = 0b1010; // 10
    let b_val = 0b0011; // 3
    let a = u8_to_bool_array(a_val);
    let b = u8_to_bool_array(b_val);

    // レジスタにデータをロード
    alu.load_a(a, true);
    alu.load_b(b, true);

    // クロック信号で操作コードを順次実行
    let clock_signals = [false, true, false, true, false, true, false, true];
    let expected_results = [0b0010, 0b1011, 0b1001, 0b1101]; // AND, OR, XOR, ADDの結果

    for (i, &clock) in clock_signals.iter().enumerate() {
        pc.update(clock);

        if clock {
            let op = pc.read(); // 現在のアドレスから操作コードを取得
            alu.execute(op, true); // 操作コードを実行

            // 結果を確認
            assert_eq!(
                bool_array_to_u8(alu.get_result()),
                expected_results[i / 2]
            );
        }
    }
}