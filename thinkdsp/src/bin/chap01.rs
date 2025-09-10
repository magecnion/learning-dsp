//! https://github.com/AllenDowney/ThinkDSP/blob/f1cc15de31f658d5df287332a30659fb16eb41d5/code/chap01.ipynb

use thinkdsp::book::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init();

    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);
    Signal::plot(
        &Sinusoid::from(cosine.clone()),
        11025,
        Some("cosine".to_string()),
    );
    Signal::plot(
        &Sinusoid::from(sine.clone()),
        11025,
        Some("sine".to_string()),
    );

    let mix = Sinusoid::from(cosine) + Sinusoid::from(sine);
    Signal::plot(&mix, 11025, Some("mix".to_string()));

    let wave = mix.make_wave(0.5, 0.0, 11025);
    log::info!("number of samples {}", wave.len());
    log::info!("timestep in ms {}", 1.0 / wave.framerate as f64 * 1000.0);

    // duda: instead of creating segment fn I created a wave smaller, what about this?
    let segment = mix.make_wave(mix.period() * 3.0, 0.0, 11025);
    segment.plot(Some("segment".to_string()));
}
