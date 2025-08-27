use std::sync::Arc;

use fundsp::hacker::*;
fn main() {
    let freq = 440.0f32;
    let amp = 0.2f32;
    let period = 1.0 / freq;

    let sine_node = sine_hz(freq) * amp;

    // Cosine = sine delayed by T/4 seconds (90°)
    let t_quarter = 1.0 / (4.0 * freq);
    let cosine_node = (sine_hz(freq) * amp) >> delay(t_quarter);

    // Sum them (reduce gain to avoid clipping)
    let sum = sine_node + cosine_node;
    let mut node = sum;

    let wave = Wave::render(11025.0, 1.0, &mut node);

    // Create a node that plays channel 0 starting at `start` seconds:
    let start_sec = 0.0;
    let len_sec = period * 3.0;
    let start_idx = (start_sec * 11025.0) as usize;
    let end_idx = start_idx + (len_sec * 11025.0) as usize;

    let mut from_offset = wavech_at(&Arc::new(wave), 0, start_idx, end_idx, None);
    let segment = Wave::render(11025.0, len_sec as f64, &mut from_offset);
    println!("Number of samples {}", segment.len());
    println!("Timestep in ms {}", 1000.0 / segment.sample_rate() as f64);

    thinkdsp::Wave::from(segment).plot(); // TODO fix it is not equal to mix_cosines plot, it might be because of waying of addition signals
}
