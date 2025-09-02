mod render;
use fundsp::hacker32::*;
use fundsp::wave::Wave as FundspWave;
use std::{
    f64::consts::TAU,
    ops::Add,
    path::Path,
    time::{self, Duration},
};

#[derive(Debug)]
pub struct Wave {
    times: Vec<Duration>, // TODO: impl esta sugerencia: Si los archivos pueden ser largos, considera no guardar times y calcular t = i as f64 / framerate as f64 cuando lo necesites (ahorra bastante RAM) + hacer benchmark
    samples: Vec<Sample>,
    pub framerate: u64,
}

impl Wave {
    pub fn plot(&self) {
        render::render(self.times.clone(), self.samples.clone());
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn new_wave_from_file<P: AsRef<Path>>(path: P) -> Result<Wave, Box<dyn std::error::Error>> {
        // TODO return proper error
        let fundsp_wave = fundsp::wave::Wave::load(path)?;
        Ok(fundsp_wave.into())
    }
}

impl From<fundsp::wave::Wave> for Wave {
    fn from(wave: fundsp::wave::Wave) -> Self {
        let framerate = wave.sample_rate() as u64;
        let n_samples = wave.len();

        let mut samples = Vec::<Sample>::with_capacity(n_samples);
        for &sample in wave.channel(0) {
            samples.push(sample as Sample);
        }

        let mut times = Vec::with_capacity(n_samples);
        for i in 0..n_samples {
            times.push(Duration::from_secs_f64(i as f64 / framerate as f64));
        }

        Wave {
            times,
            samples,
            framerate,
        }
    }
}

impl PartialEq for Wave {
    fn eq(&self, other: &Self) -> bool {
        other.len() == self.len()
            && other.framerate == self.framerate
            && other.times == self.times
            && self.samples == other.samples
    }
}

pub trait Signal {
    fn evaluate(&self, ts: &Vec<time::Duration>) -> Vec<Sample>;

    fn period(&self) -> time::Duration;

    fn make_wave(&self, duration: Duration, start: Duration, framerate: u64) -> Wave {
        let total_samples = (duration.as_secs_f64() * framerate as f64).round() as usize;
        let times: Vec<Duration> = (0..total_samples)
            .map(|i| Duration::from_secs_f64(start.as_secs_f64() + i as f64 / framerate as f64)) // TODO this is weird, refactor
            .collect();
        let samples = self.evaluate(&times);

        Wave {
            times,
            samples,
            framerate,
        }
    }
}

#[derive(Clone)]
pub struct Sinusoid {
    freq: f64,
    amp: f64,
    offset: f64,
    func: fn(f64) -> f64,
}

impl Signal for Sinusoid {
    fn evaluate(&self, times: &Vec<time::Duration>) -> Vec<Sample> {
        times
            .iter()
            .map(|&t| {
                let phase = TAU * self.freq * t.as_secs_f64() + self.offset;
                self.amp * (self.func)(phase)
            })
            .collect()
    }

    fn period(&self) -> time::Duration {
        Duration::from_secs_f64(1.0 / self.freq)
    }
}

type Sample = f64;

impl Sinusoid {
    fn new(freq: f64, amp: f64, offset: f64, func: fn(f64) -> f64) -> Sinusoid {
        Sinusoid {
            freq,
            amp,
            offset,
            func,
        }
    }
}

// duda: no tiene sentido SumSignal no? porque solo se suman sinusoid
pub struct SumSinusoid(Sinusoid, Sinusoid);
impl Signal for SumSinusoid {
    fn evaluate(&self, times: &Vec<time::Duration>) -> Vec<Sample> {
        let samples_a = self.0.evaluate(times);
        let samples_b = self.1.evaluate(times);
        samples_a
            .into_iter()
            .zip(samples_b.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }

    fn period(&self) -> time::Duration {
        self.0.period().max(self.1.period())
    }
}

impl<'a, 'b> Add<&'b Sinusoid> for &'a Sinusoid {
    type Output = SumSinusoid;

    fn add(self, rhs: &'b Sinusoid) -> Self::Output {
        SumSinusoid(self.clone(), rhs.clone())
    }
}

pub struct CosSignal(Sinusoid);

impl CosSignal {
    pub fn new(freq: f64, amp: f64, offset: f64) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f64::cos))
    }
}

impl From<CosSignal> for Sinusoid {
    fn from(c: CosSignal) -> Self {
        c.0
    }
}

pub struct SinSignal(Sinusoid);

impl SinSignal {
    pub fn new(freq: f64, amp: f64, offset: f64) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f64::sin))
    }

    fn make_wave(&self, duration: Duration, start: Duration, framerate: u64) -> Wave {
        self.0.make_wave(duration, start, framerate)
    }
}

impl From<SinSignal> for Sinusoid {
    fn from(c: SinSignal) -> Self {
        c.0
    }
}

#[test]
fn sinusoid_signal_period() {
    let s = Sinusoid::new(440.0, 1.0, 0.0, f64::sin);
    assert_eq!(s.period(), Duration::from_secs_f64(1.0 / 440.0));
}

#[test]
// TODO more tests are needed, also check real values (maybe official rust lib for audio https://rust.audio/)
fn sinusoid_signal_evaluate() {
    let s = Sinusoid::new(0.0, 0.0, 0.0, f64::sin);
    let times = vec![
        Duration::from_secs_f64(0.0),
        Duration::from_secs_f64(0.25),
        Duration::from_secs_f64(0.5),
        Duration::from_secs_f64(0.75),
    ];
    assert_eq!(s.evaluate(&times), vec![0.0, 0.0, 0.0, 0.0])
}

#[test]
fn check_framerate_for_one_sec_wave() {
    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);

    let sinusoid = &Sinusoid::from(cosine) + &Sinusoid::from(sine);
    let wave = sinusoid.make_wave(Duration::from_secs(1), Duration::from_secs(0), 11025);
    assert_eq!(wave.len(), wave.framerate as usize);
}

#[test]
fn compare_two_waves_same_len_different_framerate() {
    let wave1 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        10,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(2),
        Duration::from_secs(1),
        5,
    );
    assert_eq!(wave1.len(), wave2.len());
    assert_ne!(wave1.framerate, wave2.framerate);
    assert_ne!(wave1, wave2);
}

#[test]
fn compare_two_waves_same_len_and_framerate_different_times() {
    let wave1 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        1,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(1),
        1,
    );
    assert_eq!(wave1.len(), wave2.len());
    assert_eq!(wave1.framerate, wave2.framerate);
    assert_ne!(wave1.times, wave2.times);
    assert_ne!(wave1, wave2);
}

#[test]
fn compare_two_waves_same_len_and_framerate_different_signal() {
    let wave1 = SinSignal::new(880.0, 1.0, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        2,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        2,
    );
    assert_eq!(wave1.len(), wave2.len());
    assert_eq!(wave1.framerate, wave2.framerate);
    assert_eq!(wave1.times, wave2.times);
    assert_ne!(wave1.samples, wave2.samples);
    assert_ne!(wave1, wave2);
}

#[test]
fn compare_sine_from_fundsp() {
    // TODO check types
    let amp = 0.5_f64;
    let freq = 880.0_f64;
    let sample_rate = 11025_u64;

    let mut node = sine_hz(freq as f32) * amp as f32;

    let fundsp_wave = Wave::from(FundspWave::render(sample_rate as f64, 1.0, &mut node));
    let wave = SinSignal::new(freq, amp, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        sample_rate,
    );
    assert_eq!(fundsp_wave.samples[0], wave.samples[0]); // TODO fix
    assert_eq!(fundsp_wave.samples[1], wave.samples[1]);
    assert_eq!(fundsp_wave.samples[2], wave.samples[2]);
    assert_eq!(fundsp_wave.samples[3], wave.samples[3]);
    assert_eq!(fundsp_wave.samples[4], wave.samples[4]);
    assert_eq!(fundsp_wave, wave);
}
