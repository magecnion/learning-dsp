//! This module contains the implementation of the ThinkDSP library using the [fundsp crate](https://github.com/SamiPerttu/fundsp).
//! The reason for this is to simplify some funcion implementations taking the advantage of such a crate.

use fundsp::wave::Wave;

use crate::book;

pub fn read_wave(filename: &str) -> Result<Wave, Box<dyn std::error::Error>> {
    Wave::load(filename).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

impl From<Wave> for book::Wave {
    fn from(wave: fundsp::wave::Wave) -> Self {
        let framerate = wave.sample_rate() as f32; // TODO check this because sample_rate is returning f64 it might be a problem
        let n_samples = wave.len();

        let mut samples = Vec::<f32>::with_capacity(n_samples);
        for &sample in wave.channel(0) {
            // if it's in stereo, just pull out the first channel
            samples.push(sample);
        }

        let mut times = Vec::with_capacity(n_samples);
        for i in 0..n_samples {
            times.push(i as f32 / framerate);
        }

        book::Wave::new(samples, times, framerate as u64)
    }
}
