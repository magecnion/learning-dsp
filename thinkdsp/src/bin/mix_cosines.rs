use thinkdsp::{Signal, Sinusoid};

fn main() {
    println!("Plots three periods of a mix of consines");

    let cosine = thinkdsp::CosSignal::new(440.0, 1.0, 0.0);
    let sine = thinkdsp::SinSignal::new(880.0, 0.5, 0.0);

    let sinusoid = &Sinusoid::from(cosine) + &Sinusoid::from(sine);
    let wave = sinusoid.make_wave(1.0, 0.0, 11025);

    println!("Number of samples {}", wave.len());
    println!("Timestep in ms {}", 1000.0 / wave.framerate as f64);
    assert_eq!(wave.len(), wave.framerate as usize);
}
