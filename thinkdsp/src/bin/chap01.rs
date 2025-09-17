//! https://github.com/AllenDowney/ThinkDSP/blob/f1cc15de31f658d5df287332a30659fb16eb41d5/code/chap01.ipynb

use rodio::OutputStreamBuilder;
use thinkdsp::book::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let framerate = 11025;

    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);
    Signal::plot(
        &Sinusoid::from(cosine.clone()),
        framerate,
        Some("cosine".to_string()),
    );
    Signal::plot(
        &Sinusoid::from(sine.clone()),
        framerate,
        Some("sine".to_string()),
    );

    let mix_signals = Sinusoid::from(cosine) + Sinusoid::from(sine);
    Signal::plot(&mix_signals, framerate, Some("mix".to_string()));
    // duda: instead of creating segment fn I created a wave smaller, what about this?
    let segment = mix_signals.make_wave(mix_signals.period() * 3.0, 0.0, framerate);
    segment.plot(Some("segment".to_string())); // same as mix plot

    let wave = mix_signals.make_wave(0.5, 0.0, framerate);
    log::info!("number of samples {}", wave.len());
    log::info!("timestep in ms {}", 1.0 / wave.framerate as f64 * 1000.0);

    let wave = wave.apodize(20.0, 0.1);
    wave.plot(Some("apodized".to_string()));

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    sink.append(thinkdsp::rodio::Wave::from(wave.clone()));
    sink.sleep_until_end();

    Ok(())
}
