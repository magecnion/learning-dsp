use thinkdsp::{Signal, Sinusoid};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use std::time::Duration;

    println!("Plots three periods of a mix of sinusoids signals");

    let cosine = thinkdsp::CosSignal::new(440.0, 1.0, 0.0);
    let sine = thinkdsp::SinSignal::new(880.0, 0.5, 0.0);
    let sinusoid = &Sinusoid::from(cosine) + &Sinusoid::from(sine);
    let framerate = 11025.0;

    let wave = sinusoid.make_wave(Duration::from_secs(1), Duration::from_secs(0), framerate);

    println!("Number of samples {}", wave.len());
    println!("Timestep in ms {}", 1000.0 / wave.framerate as f64);

    // duda: instead of creating segment fn I created a wave smaller, what about this?
    let segment = sinusoid.make_wave(sinusoid.period() * 3, Duration::from_secs(0), framerate);
    segment.plot();
}
