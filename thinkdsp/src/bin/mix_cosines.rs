//! # Mix Cosines Example
//!
//! https://github.com/AllenDowney/ThinkDSP/blob/f1cc15de31f658d5df287332a30659fb16eb41d5/code/sounds.py#L95-L122

use thinkdsp::book::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Plots three periods of a mix of cosines");

    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);

    let mix = Sinusoid::from(cosine) + Sinusoid::from(sine);
    let wave = mix.make_wave(1.0, 0.0, 11025);
    println!("Number of samples {}", wave.len());
    println!("Timestep in ms {}", 1000.0 / wave.framerate as f64);
    assert_eq!(wave.len(), wave.framerate as usize);

    // duda: instead of creating segment fn I created a wave smaller, what about this?
    let segment = mix.make_wave(mix.period() * 3.0, 0.0, 11025);
    segment.plot();
}
