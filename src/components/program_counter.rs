use crate::components::{alu::{ALU4Bit, OP_LOAD, OP_STORE}, gates::{bool_array_to_u8, u8_to_bool_array}};

use super::memory::Memory;

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

    pub fn read(&self, memory: &Memory) -> u8 {
        memory.read(self.value)
    }
}

#[test]
fn test_alu_load_store_with_memory() {
    let mut memory = Memory::new(256);
    let mut pc = ProgramCounter::new();
    let mut alu = ALU4Bit::new();

    // メモリにデータをロード
    memory.write(10, 0b1100); // アドレス10にデータ12を格納

    // レジスタBにアドレス10をロード
    let address = u8_to_bool_array(10);
    alu.load_b(address, true);

    // LOAD命令を実行
    alu.execute(OP_LOAD, true, &mut memory);
    assert_eq!(bool_array_to_u8(alu.get_reg_a()), 0b1100); // レジスタAに12がロードされる

    // レジスタAに新しいデータをロード
    let new_data = u8_to_bool_array(0b1010); // データ10
    alu.load_a(new_data, true);

    // STORE命令を実行
    alu.execute(OP_STORE, true, &mut memory);
    assert_eq!(memory.read(10), 0b1010); // アドレス10にデータ10がストアされる
}