//! This module contains the official book implementation of the ThinkDSP library.
//! The intention was to remain as faithful as possible to the original implementation.

#![allow(unused)]
use std::{f32::consts::TAU, ops::Add};

use crate::gui;

/// Represents a time-varying signal.
pub trait Signal {
    /// Period of the signal.
    ///
    /// Since this is used primarily for purposes of plotting,
    /// the default behavior is to return a value, 0.1 seconds,
    /// that is reasonable for many signals.
    ///
    /// # Returns
    /// * `f32` - The period of the signal in seconds.
    fn period(&self) -> f32 {
        0.1
    }

    /// Plots the signal.
    /// The default behavior is to plot three periods.
    ///
    /// # Arguments
    /// * `framerate` - The number of samples per second.
    fn plot(&self, framerate: u64, filename: Option<String>) {
        let duration = self.period() * 3.0;
        self.make_wave(duration, 0.0, framerate).plot(filename);
    }

    /// Evaluates the signal at the given times.
    ///
    /// # Arguments
    /// * `ts` - A vector of time points (in seconds) at which to evaluate the signal.
    ///
    /// # Returns
    /// * `Vec<f32>` - The values of the signal at the given time points.
    ///
    /// NOTE: In the book's implementation subclasses as SumSinusoid implements `evaluate` and because Python is dynamic
    /// call `evaluate` from `make_wave` works, in Rust we need to be explicit and add it as part of the trait.
    fn evaluate(&self, ts: &Vec<f32>) -> Vec<f32>;

    /// Creates a wave from the signal.
    ///
    /// # Arguments
    /// * `duration` - The duration of the wave in seconds.
    /// * `start` - The start time in seconds.
    /// * `framerate` - The number of frames per second.
    ///
    /// # Returns
    /// * `Wave` - The generated wave.
    fn make_wave(&self, duration: f32, start: f32, framerate: u64) -> Wave {
        let n = (duration * framerate as f32).round() as usize;
        let ts: Vec<f32> = (0..n)
            .map(|i| start + i as f32 / framerate as f32)
            .collect();
        let ys = self.evaluate(&ts);

        Wave { ts, ys, framerate }
    }
}

/// Represents a discrete-time waveform.
pub struct Wave {
    ys: Vec<f32>,
    ts: Vec<f32>,
    pub framerate: u64,
}

impl Wave {
    /// Creates a wave.
    ///
    /// # Arguments
    /// * `ys` - wave array
    /// * `ts` - array of times
    /// * `framerate` - samples per second
    ///
    /// # Returns
    /// * `Wave` - The generated wave.
    pub fn new(ys: Vec<f32>, ts: Vec<f32>, framerate: u64) -> Self {
        Self { ys, ts, framerate }
    }

    /// Plots the wave.
    pub fn plot(&self, filename: Option<String>) {
        if let Err(e) = gui::draw(filename, self.ts.clone(), self.ys.clone()) {
            log::error!("Failed to plot wave: {}", e);
        }
    }

    /// Returns the length of the wave.
    ///
    /// # Returns
    /// * `usize` - Length of samples array.
    pub fn len(&self) -> usize {
        self.ys.len()
    }
}

/// Represents a sinusoidal signal.
#[derive(Clone)]
pub struct Sinusoid {
    freq: f32,
    amp: f32,
    offset: f32,
    func: fn(f32) -> f32,
}

impl Sinusoid {
    /// Creates a sinusoidal signal.
    ///
    /// # Arguments
    /// * `freq` - The frequency of the signal in Hz.
    /// * `amp` - The amplitude of the signal.
    /// * `offset` - The phase offset in radians.
    /// * `func` - The function that maps phase to amplitude.
    ///
    /// # Returns
    /// * `Sinusoid` - The generated sinusoidal signal.
    pub fn new(freq: f32, amp: f32, offset: f32, func: fn(f32) -> f32) -> Sinusoid {
        Sinusoid {
            freq,
            amp,
            offset,
            func,
        }
    }
}

impl Signal for Sinusoid {
    fn period(&self) -> f32 {
        1.0 / self.freq
    }

    fn evaluate(&self, ts: &Vec<f32>) -> Vec<f32> {
        ts.iter()
            .map(|&t| {
                let phase = TAU * self.freq * t + self.offset;
                self.amp * (self.func)(phase)
            })
            .collect()
    }
}

/// Represents the sum of two sinusoidal signals
///
/// NOTE: In the original implementation the `add` methods was implemenetd in `Signal` class
/// but in Rust we can't implement traits for other traits - we can only implement traits for concrete types.
impl Add for Sinusoid {
    type Output = SumSignal<Sinusoid>;

    fn add(self, other: Self) -> Self::Output {
        SumSignal(self, other)
    }
}

/// Represents a cosine sinusoid.
#[derive(Clone)]
pub struct CosSignal(Sinusoid);

impl CosSignal {
    /// Creates a cosine signal.
    ///
    /// # Arguments
    /// * `freq` - The frequency of the signal in Hz.
    /// * `amp` - The amplitude of the signal.
    /// * `offset` - The phase offset in radians.
    ///
    /// # Returns
    /// * `CosSignal` - The generated cosine signal.
    pub fn new(freq: f32, amp: f32, offset: f32) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f32::cos))
    }
}

impl From<CosSignal> for Sinusoid {
    fn from(c: CosSignal) -> Self {
        c.0
    }
}

/// Represents a sine sinusoid.
#[derive(Clone)]
pub struct SinSignal(Sinusoid);

impl SinSignal {
    /// Creates a sine signal.
    ///
    /// # Arguments
    /// * `freq` - The frequency of the signal in Hz.
    /// * `amp` - The amplitude of the signal.
    /// * `offset` - The phase offset in radians.
    ///
    /// # Returns
    /// * `SinSignal` - The generated sine signal.
    pub fn new(freq: f32, amp: f32, offset: f32) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f32::sin))
    }
}

impl From<SinSignal> for Sinusoid {
    fn from(s: SinSignal) -> Self {
        s.0
    }
}

/// Represents the sum of two signals.
pub struct SumSignal<T: Signal>(T, T);

impl<T: Signal> Signal for SumSignal<T> {
    /// Period of the signal.
    ///
    /// NOTE: this is not correct; it's mostly a placekeeper.
    ///
    /// But it is correct for a harmonic sequence where all
    /// component frequencies are multiples of the fundamental.
    ///
    /// # Returns
    /// * `f32` - The period of the signal in seconds.
    fn period(&self) -> f32 {
        self.0.period().max(self.1.period())
    }

    fn evaluate(&self, ts: &Vec<f32>) -> Vec<f32> {
        let samples_a = self.0.evaluate(ts);
        let samples_b = self.1.evaluate(ts);
        samples_a
            .into_iter()
            .zip(samples_b.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}
