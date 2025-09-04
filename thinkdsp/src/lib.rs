mod render;
use fundsp::hacker32::*;
use fundsp::wave::Wave as FundspWave;
use std::{
    f32::consts::TAU,
    ops::Add,
    path::Path,
    time::{self, Duration},
};

#[derive(Debug)]
pub struct Wave {
    times: Vec<Duration>, // TODO: impl esta sugerencia: Si los archivos pueden ser largos, considera no guardar times y calcular t = i as f32 / framerate as f32 cuando lo necesites (ahorra bastante RAM) + hacer benchmark
    samples: Vec<Sample>,
    pub framerate: f32,
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

    pub fn normalize(self, amp: f32) -> Self {
        let (mut min_sample, mut max_sample) = (self.samples[0], self.samples[0]);

        for &sample in self.samples.iter().skip(1) {
            if sample < min_sample {
                min_sample = sample;
            } else if sample > max_sample {
                max_sample = sample;
            }
        }

        let max_magnitude = min_sample.abs().max(max_sample.abs());

        Wave {
            framerate: self.framerate,
            times: self.times,
            samples: self
                .samples
                .iter()
                .map(|&sample| amp * sample / max_magnitude)
                .collect(),
        }
    }
}

impl From<fundsp::wave::Wave> for Wave {
    fn from(wave: fundsp::wave::Wave) -> Self {
        let framerate = wave.sample_rate() as f32; // TODO check this because sample_rate is returning f64
        let n_samples = wave.len();

        let mut samples = Vec::<Sample>::with_capacity(n_samples);
        for &sample in wave.channel(0) {
            // if it's in stereo, just pull out the first channel
            samples.push(sample as Sample);
        }

        let mut times = Vec::with_capacity(n_samples);
        for i in 0..n_samples {
            times.push(Duration::from_secs_f32(i as f32 / framerate));
        }

        Wave {
            times,
            samples,
            framerate,
        }
    }
}

const EPS: f32 = 1e-3;

impl PartialEq for Wave {
    fn eq(&self, other: &Self) -> bool {
        other.len() == self.len()
            && other.framerate == self.framerate
            && other.times == self.times
            && self
                .samples
                .iter()
                .zip(other.samples.iter())
                .all(|(a, b)| (a - b).abs() <= EPS)
    }
}

pub trait Signal {
    fn evaluate(&self, ts: &Vec<time::Duration>) -> Vec<Sample>;

    fn period(&self) -> time::Duration;

    fn make_wave(&self, duration: Duration, start: Duration, framerate: f32) -> Wave {
        let total_samples = (duration.as_secs_f32() * framerate as f32).round() as usize;
        let times: Vec<Duration> = (0..total_samples)
            .map(|i| Duration::from_secs_f32(start.as_secs_f32() + i as f32 / framerate as f32)) // TODO this is weird, refactor
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
    freq: f32,
    amp: f32,
    offset: f32,
    func: fn(f32) -> f32,
}

impl Signal for Sinusoid {
    fn evaluate(&self, times: &Vec<time::Duration>) -> Vec<Sample> {
        times
            .iter()
            .map(|&t| {
                let phase = TAU * self.freq * t.as_secs_f32() + self.offset;
                self.amp * (self.func)(phase)
            })
            .collect()
    }

    fn period(&self) -> time::Duration {
        Duration::from_secs_f32(1.0 / self.freq)
    }
}

type Sample = f32;

impl Sinusoid {
    fn new(freq: f32, amp: f32, offset: f32, func: fn(f32) -> f32) -> Sinusoid {
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
    pub fn new(freq: f32, amp: f32, offset: f32) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f32::cos))
    }
}

impl From<CosSignal> for Sinusoid {
    fn from(c: CosSignal) -> Self {
        c.0
    }
}

#[derive(Clone)]
pub struct SinSignal(Sinusoid);

impl SinSignal {
    pub fn new(freq: f32, amp: f32, offset: f32) -> Self {
        Self(Sinusoid::new(freq, amp, offset, f32::sin))
    }

    pub fn make_wave(&self, duration: Duration, start: Duration, framerate: f32) -> Wave {
        self.0.make_wave(duration, start, framerate)
    }

    pub fn period(&self) -> time::Duration {
        self.0.period()
    }
}

impl From<SinSignal> for Sinusoid {
    fn from(c: SinSignal) -> Self {
        c.0
    }
}

#[test]
fn sinusoid_signal_period() {
    let s = Sinusoid::new(440.0, 1.0, 0.0, f32::sin);
    assert_eq!(s.period(), Duration::from_secs_f32(1.0 / 440.0));
}

#[test]
// TODO more tests are needed, also check real values (maybe official rust lib for audio https://rust.audio/)
fn sinusoid_signal_evaluate() {
    let s = Sinusoid::new(0.0, 0.0, 0.0, f32::sin);
    let times = vec![
        Duration::from_secs_f32(0.0),
        Duration::from_secs_f32(0.25),
        Duration::from_secs_f32(0.5),
        Duration::from_secs_f32(0.75),
    ];
    assert_eq!(s.evaluate(&times), vec![0.0, 0.0, 0.0, 0.0])
}

#[test]
fn check_framerate_for_one_sec_wave() {
    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    let sine = SinSignal::new(880.0, 0.5, 0.0);

    let sinusoid = &Sinusoid::from(cosine) + &Sinusoid::from(sine);
    let wave = sinusoid.make_wave(Duration::from_secs(1), Duration::from_secs(0), 11025.0);
    assert_eq!(wave.len(), wave.framerate as usize);
}

#[test]
fn compare_two_waves_same_len_different_framerate() {
    let wave1 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        10.0,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(2),
        Duration::from_secs(1),
        5.0,
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
        1.0,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(1),
        1.0,
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
        2.0,
    );
    let wave2 = SinSignal::new(880.0, 0.5, 0.0).make_wave(
        Duration::from_secs(1),
        Duration::from_secs(0),
        2.0,
    );
    assert_eq!(wave1.len(), wave2.len());
    assert_eq!(wave1.framerate, wave2.framerate);
    assert_eq!(wave1.times, wave2.times);
    assert_ne!(wave1.samples, wave2.samples);
    assert_ne!(wave1, wave2);
}

#[test]
fn compare_sine_from_fundsp() {
    let amp = 0.5_f32;
    let freq = 880.0_f32;
    let sample_rate = 11025.0_f32;
    let duration = 1.0_f32;
    let start = 0.0_f32;

    let mut node = constant(freq) >> sine_phase(0.0) * amp;
    let fundsp_wave = Wave::from(FundspWave::render(
        sample_rate as f64, // TODO this is weird
        duration as f64,    // TODO this is weird
        &mut node,
    ));

    let wave = SinSignal::new(freq, amp, 0.0).make_wave(
        Duration::from_secs_f32(duration),
        Duration::from_secs_f32(start),
        sample_rate,
    );

    // debugging purpose so you can check how much is the epsilon
    for i in 0..fundsp_wave.len() {
        let (a_time, a_sample) = (fundsp_wave.times[i].as_secs_f32(), fundsp_wave.samples[i]);
        let (b_time, b_sample) = (wave.times[i].as_secs_f32(), wave.samples[i]);
        assert!(
            (a_sample.abs() - b_sample.abs()) < EPS,
            "Mismatch at sample {i}: fundsp={a_sample}, custom={b_sample}"
        );
        assert!(
            (a_time.abs() - b_time.abs()) < EPS,
            "Mismatch at sample {i}: fundsp={a_time}, custom={b_time}"
        );
    }
    assert_eq!(fundsp_wave, wave);
}
