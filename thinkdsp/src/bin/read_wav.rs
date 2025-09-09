use thinkdsp::book;
use thinkdsp::fundsp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let path = format!(
        "{}/presets/268050__sceza__bass-sine-sweep-10-50hz.wav",
        env!("CARGO_MANIFEST_DIR")
    );

    let wave: book::Wave = fundsp::read_wave(&path).unwrap().into();
    println!("Number of samples {}", wave.len());
    println!("Timestep in ms {}", 1000.0 / wave.framerate as f64);
    println!("frame/sample rate {}", wave.framerate);
    wave.plot();
    // wave.normalize(1.0).plot(); // TODO
}
