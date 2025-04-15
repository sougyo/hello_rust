mod components;

use components::{
    adders::{half_adder, full_adder, four_bit_adder},
    alu::ALU4Bit,
    memory::{DFlipFlop},
    program_counter::ProgramCounter,
    gates::{u8_to_bool_array, bool_array_to_u8},
};

fn main() {
    // プログラムカウンタの初期化
    let mut pc = ProgramCounter::new();

    // クロック信号のシミュレーション
    let clock_signals = [false, true, false, true, false, true, false, true];

    println!("=== プログラムカウンタのテスト ===");
    println!("Clock\tPC");

    for &clock in &clock_signals {
        pc.update(clock);
        println!("{}\t{}", clock, pc.get());
    }

    // 1. 半加算器のテスト
    println!("=== 半加算器のテスト ===");
    let inputs = [(false, false), (false, true), (true, false), (true, true)];
    println!("A\tB\tSum\tCarry");
    for (a, b) in inputs.iter() {
        let (sum, carry) = half_adder(*a, *b);
        println!("{}\t{}\t{}\t{}", a, b, sum, carry);
    }
    
    // 2. 全加算器のテスト
    println!("\n=== 全加算器のテスト ===");
    println!("A\tB\tCin\tSum\tCout");
    for (a, b) in inputs.iter() {
        for carry_in in [false, true].iter() {
            let (sum, carry_out) = full_adder(*a, *b, *carry_in);
            println!("{}\t{}\t{}\t{}\t{}", a, b, carry_in, sum, carry_out);
        }
    }
    
    // 3. 4ビット加算器のテスト
    println!("\n=== 4ビット加算器のテスト ===");
    let test_cases = [
        (0b0101, 0b0011), // 5 + 3 = 8
        (0b1010, 0b0101), // 10 + 5 = 15
        (0b1111, 0b0001), // 15 + 1 = 16 (オーバーフロー)
    ];
    
    println!("A\tB\tSum\tCarry");
    for (a_val, b_val) in test_cases.iter() {
        let a = u8_to_bool_array(*a_val);
        let b = u8_to_bool_array(*b_val);
        let (sum, carry) = four_bit_adder(a, b);
        println!("{}\t{}\t{}\\t{}", a_val, b_val, bool_array_to_u8(sum), carry);
    }
    
    // 4. D型フリップフロップのテスト
    println!("\n=== D型フリップフロップのテスト ===");
    let mut dff = DFlipFlop::new();
    
    println!("Clock\tData\tOutput");
    for &(clock, data) in &[(false, true), (true, true), (false, false), (true, false), (false, true), (true, true)] {
        dff.update(data, clock);
        println!("{}\t{}\t{}", clock, data, dff.output());
    }
    
    // 5. ALUのテスト
    println!("\n=== ALUのテスト ===");
    let mut alu = ALU4Bit::new();
    
    // テストデータ
    let a_val = 0b1010; // 10
    let b_val = 0b0011; // 3
    let a = u8_to_bool_array(a_val);
    let b = u8_to_bool_array(b_val);
    
    // レジスタにデータをロード
    alu.load_a(a, true);
    alu.load_b(b, true);
    
    println!("操作\t結果");
    
    // AND操作
    alu.execute(0, true);
    println!("AND\t{}", bool_array_to_u8(alu.get_result())); // 10 AND 3 = 2

    alu.execute(0, false);

    // OR操作
    alu.execute(1, true);
    println!("OR\t{}", bool_array_to_u8(alu.get_result())); // 10 OR 3 = 11
    
    alu.execute(0, false);

    // XOR操作
    alu.execute(2, true);
    println!("XOR\t{}", bool_array_to_u8(alu.get_result())); // 10 XOR 3 = 9
    
    alu.execute(0, false);

    // 加算操作
    alu.execute(3, true);
    println!("ADD\t{}", bool_array_to_u8(alu.get_result())); // 10 + 3 = 13

    alu.execute(0, false);

    alu.execute(4, true); // 未定義の操作
    println!("未定義の操作\t{}", bool_array_to_u8(alu.get_result())); // 結果は変わらない
    // ALUの状態を確認
    println!("ALUの状態: A={}, B={}, Result={}", 
        bool_array_to_u8(alu.get_reg_a()), 
        bool_array_to_u8(alu.get_reg_b()), 
        bool_array_to_u8(alu.get_result()));
}