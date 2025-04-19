mod components;

use components::{
    program_counter::ProgramCounter,
    alu::{ALU4Bit, OP_AND, OP_OR, OP_XOR, OP_ADD},
    gates::{u8_to_bool_array, bool_array_to_u8},
    memory::Memory,
};

fn main() {
    // メモリの初期化（256バイトのメモリを持つ）
    let mut memory = Memory::new(256);

    // プログラムカウンタの初期化
    let mut pc = ProgramCounter::new();

    // メモリに操作コードをロード
    memory.write(0, OP_AND); // AND操作
    memory.write(1, OP_OR);  // OR操作
    memory.write(2, OP_XOR); // XOR操作
    memory.write(3, OP_ADD); // 加算操作

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
            let op = pc.read(&memory); // 現在のアドレスから操作コードを取得
            alu.execute(op, true, &mut memory); // 操作コードを実行
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