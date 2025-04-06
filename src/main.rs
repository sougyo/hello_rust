fn main() {
    let nx = 100; // 空間分割数
    let nt = 1000; // 時間ステップ数
    let dx = 1.0 / (nx as f64);
    let dt = 0.0005;
    let alpha = 0.01;

    let mut u = vec![0.0; nx];
    let mut u_new = vec![0.0; nx];

    // 初期条件: 中央に熱源
    u[nx / 2] = 100.0;

    for _ in 0..nt {
        for i in 1..nx - 1 {
            u_new[i] = u[i] + alpha * dt / (dx * dx) * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
        }

        // 更新
        std::mem::swap(&mut u, &mut u_new);
    }

    // 結果の表示（簡易）
    for (i, val) in u.iter().enumerate() {
        println!("{:.3}: {:.3}", i as f64 * dx, val);
    }
}
