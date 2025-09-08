//! This module contains the dummy implementation of the ThinkDSP library.
//! It served as starting point to implement the library in Rust.

#![allow(unused)]
use std::ops::Add;

/// Represents a time-varying signal.
pub trait Signal {
    /// Period of the signal.
    ///
    /// # Returns
    /// * `f32` - The period of the signal in seconds.
    fn period(&self) -> f32;

    /// Plots the signal.
    ///
    /// # Arguments
    /// * `framerate` - The number of samples per second.
    fn plot(&self, framerate: u64) {
        unimplemented!()
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
    // fn evaluate(&self, ts: &Vec<f32>) -> Vec<f32>;

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
        unimplemented!()
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
        unimplemented!()
    }

    /// Plots the wave.
    pub fn plot(&self) {
        unimplemented!()
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
        unimplemented!()
    }
}

impl Signal for Sinusoid {
    /// Period of the signal.
    ///
    /// # Returns
    /// * `f32` - The period of the signal in seconds.
    fn period(&self) -> f32 {
        unimplemented!()
    }
}

/// Represents the sum of two sinusoidal signals
///
/// NOTE: In the original implementation the `add` methods was implemenetd in `Signal` class
/// but in Rust we can't implement traits for other traits - we can only implement traits for concrete types.
impl Add for Sinusoid {
    type Output = SumSignal<Sinusoid>;

    fn add(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}

/// Represents a cosine Sinusoid.
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
        unimplemented!()
    }
}

impl From<CosSignal> for Sinusoid {
    fn from(c: CosSignal) -> Self {
        c.0
    }
}

/// Represents a cosine Sinusoid.
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
        unimplemented!()
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
    /// Period of the signal in seconds.
    ///
    /// # Returns
    /// * `f32` - The period of the signal in seconds.
    fn period(&self) -> f32 {
        unimplemented!()
    }
}
