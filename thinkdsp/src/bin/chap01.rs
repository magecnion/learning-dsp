//! https://github.com/AllenDowney/ThinkDSP/blob/f1cc15de31f658d5df287332a30659fb16eb41d5/code/chap01.ipynb

use rodio::OutputStreamBuilder;
use thinkdsp::{book::*, fundsp};

const FRAMERATE: u64 = 11025;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // SIGNALS

    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);
    Signal::plot(
        &Sinusoid::from(cosine.clone()),
        FRAMERATE,
        Some("cosine".to_string()),
    );
    Signal::plot(
        &Sinusoid::from(sine.clone()),
        FRAMERATE,
        Some("sine".to_string()),
    );

    let mix_signals = Sinusoid::from(cosine) + Sinusoid::from(sine);
    Signal::plot(&mix_signals, FRAMERATE, Some("mix".to_string()));
    // duda: instead of creating segment fn I created a wave smaller, what about this?
    let segment = mix_signals.make_wave(mix_signals.period() * 3.0, 0.0, FRAMERATE);
    segment.plot(Some("segment".to_string())); // same as mix plot

    // WAVES

    let wave = mix_signals.make_wave(0.5, 0.0, FRAMERATE);
    log::info!("number of samples {}", wave.len());
    log::info!("timestep in ms {}", 1.0 / wave.framerate as f64 * 1000.0);

    let wave = wave.apodize(20.0, 0.1);
    // TODO normalize, segment
    wave.plot(Some("apodized".to_string()));

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    sink.append(thinkdsp::rodio::Wave::from(wave.clone()));
    sink.sleep_until_end();

    // TODO normalize, segment, write_wave

    let path = format!(
        "{}/presets/268050__sceza__bass-sine-sweep-10-50hz.wav",
        env!("CARGO_MANIFEST_DIR")
    );
    let wave: Wave = fundsp::read_wave(&path).unwrap().into();
    sink.append(thinkdsp::rodio::Wave::from(wave.clone()));
    sink.sleep_until_end();

    Ok(())
}
