use super::gates::{and_gate, or_gate, xor_gate};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor_gate(a, b);
    let carry = and_gate(a, b);
    (sum, carry)
}

pub fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = or_gate(carry1, carry2);
    (sum, carry_out)
}

pub fn four_bit_adder(a: [bool; 4], b: [bool; 4]) -> ([bool; 4], bool) {
    let mut result = [false; 4];
    let mut carry = false;

    for i in 0..4 {
        let (sum, new_carry) = full_adder(a[i], b[i], carry);
        result[i] = sum;
        carry = new_carry;
    }

    (result, carry)
}