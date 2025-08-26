mod render;
use std::{
    f64::consts::TAU,
    ops::Add,
    path::Path,
    time::{self, Duration},
};

pub struct Wave {
    times: Vec<Duration>,
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

    // TODO: impl From fundsp::Wave to my Wave
    pub fn new_wave_from_file<P: AsRef<Path>>(path: P) -> Result<Wave, Box<dyn std::error::Error>> {
        // TODO return proper error
        let fundsp_wave = fundsp::wave::Wave::load(path)?;
        let framerate = fundsp_wave.sample_rate() as u64;
        let n_samples = fundsp_wave.len();

        let mut samples = Vec::<Sample>::with_capacity(n_samples);
        for &sample in fundsp_wave.channel(0) {
            samples.push(sample as Sample);
        }

        let mut times = Vec::with_capacity(n_samples);
        for i in 0..n_samples {
            times.push(Duration::from_secs_f64(i as f64 / framerate as f64));
        }

        Ok(Wave {
            times,
            samples,
            framerate,
        })
    }
}

pub trait Signal {
    fn evaluate(&self, ts: &Vec<time::Duration>) -> Vec<Sample>;

    fn period(&self) -> time::Duration;

    // TODO create a new fn in Wave struct: fn new_wave_from_signal(s: Signal) -> Wave { ... }
    fn make_wave(&self, duration: Duration, start: Duration, framerate: u64) -> Wave {
        let samples = (duration.as_secs_f64() * framerate as f64).round() as usize;
        let times: Vec<Duration> = (0..samples)
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
