use cudarc::{driver::{CudaContext, DriverError, LaunchConfig, PushKernelArg}, nvrtc::Ptx};
use anyhow::Result;
use std::time::Instant;

fn main() -> Result<()> {
    let ctx = CudaContext::new(0)?;
    let stream = ctx.default_stream();

    let module = ctx.load_module(Ptx::from_file("src/kernel/constant_memory.ptx"))?;

    let mut coefficients_symbol = module.get_global("coefficients", &stream)?;
    
    let coefficients = [1.0f32, 2.0, 3.0, 4.0];

    let mut symbol_view = coefficients_symbol.as_view_mut();
    let mut symbol_f32 = unsafe {
        symbol_view.transmute_mut::<f32>(4).unwrap()
    };
    stream.memcpy_htod(&coefficients, &mut symbol_f32)?;

    let polynoimal_kernel = module.load_function("polynomial_kernel")?;

    // let input = vec![0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0];
    let input: Vec<f32> = (0..16384).map(|i| i as f32).collect();
    let n = input.len();

    let input_dev = stream.clone_htod(&input)?;
    let mut output_dev = stream.alloc_zeros::<f32>(n)?;

    let cfg = LaunchConfig::for_num_elems(n as u32);

    // --- ベンチマーク設定 ---
    let iterations = 50000; // ループ回数

    // 1. ウォームアップ（最初の1回は初期化コストがかかることがあるため、計測に含めない）
    unsafe {
        stream.launch_builder(&polynoimal_kernel)
            .arg(&mut output_dev)
            .arg(&input_dev)
            .arg(&(n as i32))
            .launch(cfg)?;
    }
    stream.synchronize()?; // ウォームアップ完了を待つ

    println!("Start benchmarking for {} iterations...", iterations);

    // 2. 計測開始
    let start = Instant::now();

    for _ in 0..iterations {
        unsafe {
            // launch_builderは再利用可能です
            stream.launch_builder(&polynoimal_kernel)
                .arg(&mut output_dev) // 同じメモリ領域を使い回す
                .arg(&input_dev)
                .arg(&(n as i32))
                .launch(cfg)?;
        }
        // ここで synchronize() を呼ぶと「毎回」CPUとGPUが同期して遅くなるため、
        // スループットを見たい場合はループの外で最後に1回待つのが一般的です。
    }

    // 3. 全てのキューが処理されるのを待つ (重要！)
    stream.synchronize()?;

    // 4. 計測終了
    let duration = start.elapsed();
    
    // 結果表示
    let total_time_ms = duration.as_millis();
    let avg_time_us = duration.as_micros() as f64 / iterations as f64;

    println!("Total time: {} ms", total_time_ms);
    println!("Average kernel time: {:.3} µs / iter", avg_time_us);

    // データの検証（最後の実行結果を取得）
    let output = stream.clone_dtoh(&output_dev)?;
    println!("Output sample (first 5): {:?}", &output[0..5]);

    Ok(())
}