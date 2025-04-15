pub fn and_gate(a: bool, b: bool) -> bool { a && b }
pub fn or_gate(a: bool, b: bool) -> bool { a || b }
pub fn nand_gate(a: bool, b: bool) -> bool { !(a && b) }
pub fn xor_gate(a: bool, b: bool) -> bool { a != b }
pub fn not_gate(a: bool) -> bool { !a }

pub fn bool_array_to_u8(arr: [bool; 4]) -> u8 {
    let mut result = 0;
    for i in 0..4 {
        if arr[i] {
            result |= 1 << i;
        }
    }
    result
}

pub fn u8_to_bool_array(value: u8) -> [bool; 4] {
    let mut result = [false; 4];
    for i in 0..4 {
        result[i] = (value & (1 << i)) != 0;
    }
    result
}