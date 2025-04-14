// Rust による CPU コンポーネントのシミュレーション

// 基本論理ゲート
fn and_gate(a: bool, b: bool) -> bool { a && b }
fn or_gate(a: bool, b: bool) -> bool { a || b }
fn nand_gate(a: bool, b: bool) -> bool { !(a && b) }
fn xor_gate(a: bool, b: bool) -> bool { a != b }
fn not_gate(a: bool) -> bool { !a }

// === 1. 半加算器と全加算器 ===

/// 半加算器: 2つの1ビット入力を加算し、和(sum)と桁上げ(carry)を出力
fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor_gate(a, b);     // 和はXOR
    let carry = and_gate(a, b);   // 桁上げはAND
    (sum, carry)
}

/// 全加算器: 3つの1ビット入力(a, b, 前の桁上げ)を加算し、和と桁上げを出力
fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = or_gate(carry1, carry2);
    (sum, carry_out)
}

// === 2. 4ビット加算器 ===

/// 4ビット加算器: 2つの4ビット数値を加算し、結果と桁上げを出力
fn four_bit_adder(a: [bool; 4], b: [bool; 4]) -> ([bool; 4], bool) {
    let mut result = [false; 4];
    let mut carry = false;
    
    // 各ビットを加算
    for i in 0..4 {
        let (sum, new_carry) = full_adder(a[i], b[i], carry);
        result[i] = sum;
        carry = new_carry;
    }
    
    (result, carry)
}

// === 3. メモリ要素（D型フリップフロップとラッチ） ===

/// D型ラッチ: データ入力とイネーブル信号に基づいて状態を保持
struct DLatch {
    state: bool,
}

impl DLatch {
    fn new() -> Self {
        DLatch { state: false }
    }
    
    /// データ入力とイネーブル信号を受け取り、必要に応じて状態を更新
    fn update(&mut self, data: bool, enable: bool) {
        if enable {
            self.state = data;
        }
    }
    
    /// 現在の状態を取得
    fn output(&self) -> bool {
        self.state
    }
}

/// D型フリップフロップ: クロックの立ち上がりエッジでデータをキャプチャ
struct DFlipFlop {
    state: bool,
    prev_clock: bool,
}

impl DFlipFlop {
    fn new() -> Self {
        DFlipFlop {
            state: false,
            prev_clock: false,
        }
    }
    
    /// クロックとデータ入力を受け取り、クロックの立ち上がりエッジで状態を更新
    fn update(&mut self, data: bool, clock: bool) {
        // クロックの立ち上がりエッジを検出 (前回false、今回true)
        if clock && !self.prev_clock {
            self.state = data;
        }
        self.prev_clock = clock;
    }
    
    /// 現在の状態を取得
    fn output(&self) -> bool {
        self.state
    }
}

// === 4. レジスタ ===

/// 4ビットレジスタ: 4ビットのデータを格納
struct Register4Bit {
    flip_flops: [DFlipFlop; 4],
}

impl Register4Bit {
    fn new() -> Self {
        Register4Bit {
            flip_flops: [
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
                DFlipFlop::new(),
            ],
        }
    }
    
    /// データをレジスタに読み込む
    fn load(&mut self, data: [bool; 4], clock: bool) {
        for i in 0..4 {
            self.flip_flops[i].update(data[i], clock);
        }
    }
    
    /// レジスタの内容を取得
    fn output(&self) -> [bool; 4] {
        let mut result = [false; 4];
        for i in 0..4 {
            result[i] = self.flip_flops[i].output();
        }
        result
    }
}

// === 5. ALU（算術論理演算ユニット） ===

/// 簡単な4ビットALU
struct ALU4Bit {
    // レジスタA、B、結果
    reg_a: Register4Bit,
    reg_b: Register4Bit,
    result_reg: Register4Bit,
}

impl ALU4Bit {
    fn new() -> Self {
        ALU4Bit {
            reg_a: Register4Bit::new(),
            reg_b: Register4Bit::new(),
            result_reg: Register4Bit::new(),
        }
    }
    
    /// レジスタAにデータをロード
    fn load_a(&mut self, data: [bool; 4], clock: bool) {
        self.reg_a.load(data, clock);
    }
    
    /// レジスタBにデータをロード
    fn load_b(&mut self, data: [bool; 4], clock: bool) {
        self.reg_b.load(data, clock);
    }
    
    /// ALU操作を実行
    /// op_code: 0=AND, 1=OR, 2=XOR, 3=加算
    fn execute(&mut self, op_code: u8, clock: bool) {
        let a = self.reg_a.output();
        let b = self.reg_b.output();
        let mut result = [false; 4];
        
        match op_code {
            0 => { // AND
                for i in 0..4 {
                    result[i] = and_gate(a[i], b[i]);
                }
            },
            1 => { // OR
                for i in 0..4 {
                    result[i] = or_gate(a[i], b[i]);
                }
            },
            2 => { // XOR
                for i in 0..4 {
                    result[i] = xor_gate(a[i], b[i]);
                }
            },
            3 => { // 加算
                let (sum, _) = four_bit_adder(a, b);
                result = sum;
            },
            _ => {} // 未定義の操作
        }
        
        // 結果をレジスタに格納
        self.result_reg.load(result, clock);
    }
    
    /// 結果レジスタの内容を取得
    fn get_result(&self) -> [bool; 4] {
        self.result_reg.output()
    }
}

// ヘルパー関数: ブール配列を数値に変換
fn bool_array_to_u8(arr: [bool; 4]) -> u8 {
    let mut result = 0;
    for i in 0..4 {
        if arr[i] {
            result |= 1 << i;
        }
    }
    result
}

// ヘルパー関数: 数値をブール配列に変換
fn u8_to_bool_array(value: u8) -> [bool; 4] {
    let mut result = [false; 4];
    for i in 0..4 {
        result[i] = (value & (1 << i)) != 0;
    }
    result
}

fn main() {
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
        println!("{}\t{}\t{}\t{}", a_val, b_val, bool_array_to_u8(sum), carry);
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
        bool_array_to_u8(alu.reg_a.output()), 
        bool_array_to_u8(alu.reg_b.output()), 
        bool_array_to_u8(alu.get_result()));

}

#[cfg(test)]
mod tests {
    use super::*;

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
}