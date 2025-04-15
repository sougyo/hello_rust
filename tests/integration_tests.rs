use hello_rust::components::{
    adders::{half_adder, full_adder, four_bit_adder},
    alu::ALU4Bit,
    memory::DFlipFlop,
    program_counter::ProgramCounter,
    gates::{u8_to_bool_array, bool_array_to_u8},
};

#[test]
fn test_program_counter() {
    let mut pc = ProgramCounter::new();

    // 初期値は0
    assert_eq!(pc.get(), 0);

    // クロック信号でインクリメント
    pc.update(false); // クロックがfalseでは変化しない
    assert_eq!(pc.get(), 0);

    pc.update(true); // クロックの立ち上がりでインクリメント
    assert_eq!(pc.get(), 1);

    pc.update(false); // クロックがfalseでは変化しない
    assert_eq!(pc.get(), 1);

    pc.update(true); // 再度クロックの立ち上がりでインクリメント
    assert_eq!(pc.get(), 2);

    // オーバーフローの確認
    pc.set(u16::MAX);
    pc.update(false);
    pc.update(true); // 立ち上がりでオーバーフロー
    assert_eq!(pc.get(), 0);
}

#[test]
fn test_half_adder() {
    let inputs = [(false, false), (false, true), (true, false), (true, true)];
    for (a, b) in inputs.iter() {
        let (sum, carry) = half_adder(*a, *b);
        assert_eq!(sum, a ^ b);
        assert_eq!(carry, *a && *b);
    }
}

#[test]
fn test_full_adder() {
    let inputs = [(false, false), (false, true), (true, false), (true, true)];
    for (a, b) in inputs.iter() {
        for carry_in in [false, true].iter() {
            let (sum, carry_out) = full_adder(*a, *b, *carry_in);
            assert_eq!(sum, (*a ^ *b) ^ *carry_in);
            assert_eq!(carry_out, (*a && *b) || ((*a ^ *b) && *carry_in));
        }
    }
}

#[test]
fn test_four_bit_adder() {
    let test_cases = [
        (0b0101, 0b0011, 0b1000, false), // 5 + 3 = 8
        (0b1010, 0b0101, 0b1111, false), // 10 + 5 = 15
        (0b1111, 0b0001, 0b0000, true),  // 15 + 1 = 16 (オーバーフロー)
    ];

    for (a_val, b_val, expected_sum, expected_carry) in test_cases.iter() {
        let a = u8_to_bool_array(*a_val);
        let b = u8_to_bool_array(*b_val);
        let (sum, carry) = four_bit_adder(a, b);
        assert_eq!(bool_array_to_u8(sum), *expected_sum);
        assert_eq!(carry, *expected_carry);
    }
}

#[test]
fn test_d_flip_flop() {
    let mut dff = DFlipFlop::new();
    let test_cases = [
        (false, true, false),
        (true, true, true),
        (false, false, true),
        (true, false, false),
        (false, true, false),
        (true, true, true),
    ];

    for &(clock, data, expected_output) in &test_cases {
        dff.update(data, clock);
        assert_eq!(dff.output(), expected_output);
    }
}

#[test]
fn test_alu() {
    let mut alu = ALU4Bit::new();

    let a_val = 0b1010; // 10
    let b_val = 0b0011; // 3
    let a = u8_to_bool_array(a_val);
    let b = u8_to_bool_array(b_val);

    alu.load_a(a, true);
    alu.load_b(b, true);

    // AND操作
    alu.execute(0, false);
    alu.execute(0, true);
    assert_eq!(bool_array_to_u8(alu.get_result()), 0b0010); // 10 AND 3 = 2

    // OR操作
    alu.execute(0, false);
    alu.execute(1, true);
    assert_eq!(bool_array_to_u8(alu.get_result()), 0b1011); // 10 OR 3 = 11

    // XOR操作
    alu.execute(0, false);
    alu.execute(2, true);
    assert_eq!(bool_array_to_u8(alu.get_result()), 0b1001); // 10 XOR 3 = 9

    // 加算操作
    alu.execute(0, false);
    alu.execute(3, true);
    assert_eq!(bool_array_to_u8(alu.get_result()), 0b1101); // 10 + 3 = 13
}