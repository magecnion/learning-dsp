use std::{
    f64::consts::TAU,
    ops::Add,
    time::{self, Duration},
};

pub struct Sinusoid {
    freq: f64,
    amp: f64,
    offset: f64,
    func: fn(f64) -> f64,
}

type AirPressure = f64;

impl Sinusoid {
    pub fn new(freq: f64, amp: f64, offset: f64, func: fn(f64) -> f64) -> Sinusoid {
        Sinusoid {
            freq,
            amp,
            offset,
            func,
        }
    }

    fn period(&self) -> time::Duration {
        Duration::from_secs_f64(1.0 / self.freq)
    }

    pub fn evaluate(&self, times: &Vec<time::Duration>) -> Vec<AirPressure> {
        times
            .iter()
            .map(|&t| {
                let phase = TAU * self.freq * t.as_secs_f64() + self.offset;
                self.amp * (self.func)(phase)
            })
            .collect()
    }
}

impl<'a, 'b> Add<&'b Sinusoid> for &'a Sinusoid {
    type Output = Sinusoid;

    fn add(self, _rhs: &'b Sinusoid) -> Self::Output {
        unimplemented!()
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
