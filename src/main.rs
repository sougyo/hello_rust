mod components;

use components::{
    program_counter::ProgramCounter,
    alu::ALU4Bit,
    gates::{u8_to_bool_array, bool_array_to_u8},
};

fn main() {
    // プログラムカウンタの初期化（メモリサイズを指定）
    let mut pc = ProgramCounter::new(256); // 256バイトのメモリを持つ

    // メモリに操作コードをロード
    pc.load_memory(0, 0); // AND操作
    pc.load_memory(1, 1); // OR操作
    pc.load_memory(2, 2); // XOR操作
    pc.load_memory(3, 3); // 加算操作

    // ALUの初期化
    let mut alu = ALU4Bit::new();

    // テストデータ
    let a_val = 0b1010; // 10
    let b_val = 0b0011; // 3
    let a = u8_to_bool_array(a_val);
    let b = u8_to_bool_array(b_val);

    // レジスタにデータをロード
    alu.load_a(a, true);
    alu.load_b(b, true);

    // クロック信号のシミュレーション
    let clock_signals = [false, true, false, true, false, true, false, true];

    println!("=== プログラムカウンタとALUのテスト ===");
    println!("Clock\tPC\tOp\tResult");

    for &clock in &clock_signals {
        pc.update(clock);

        if clock {
            let op = pc.read(); // 現在のアドレスから操作コードを取得
            alu.execute(op, true); // 操作コードを実行
            println!(
                "{}\t{}\t{}\t{}",
                clock,
                pc.get(),
                op,
                bool_array_to_u8(alu.get_result())
            );
        }
    }
}