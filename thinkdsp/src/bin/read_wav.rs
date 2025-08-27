#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use thinkdsp::Wave;

    let path = format!("{}/presets/tecla_audio.wav", env!("CARGO_MANIFEST_DIR"));

    let wave = Wave::new_wave_from_file(path).unwrap();
    println!("Number of samples {}", wave.len());
    println!("Timestep in ms {}", 1000.0 / wave.framerate as f64);
    println!("frame/sample rate {}", wave.framerate);
    wave.plot();
}
