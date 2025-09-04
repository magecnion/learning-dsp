use fundsp::hacker32::*;
use std::sync::Arc;

fn main() {
    let freq = 440.0f32;
    let amp = 0.2f32;
    let period = 1.0 / freq;
    let duration = period * 3.0;

    let sine_node = constant(880.0) >> sine_phase(0.0) * 0.5;
    let cosine_node = constant(440.0) >> sine_phase(std::f32::consts::FRAC_PI_2) * 1.0;

    // Sum them (reduce gain to avoid clipping)
    let sum = sine_node + cosine_node;
    let mut node = sum;

    let wave = Wave::render(11025.0, duration as f64, &mut node);

    // Create a node that plays channel 0 starting at `start` seconds:
    let start_sec = 0.0;
    let start_idx = (start_sec * 11025.0) as usize;
    let end_idx = start_idx + (duration * 11025.0) as usize;

    let mut from_offset = wavech_at(&Arc::new(wave), 0, start_idx, end_idx, None);
    let segment = Wave::render(11025.0, duration as f64, &mut from_offset);
    println!("Number of samples {}", segment.len());
    println!("Timestep in ms {}", 1000.0 / segment.sample_rate() as f64);

    thinkdsp::Wave::from(segment).plot(); // TODO fix it is not equal to mix_cosines plot, it might be because of waying of addition signals
}
