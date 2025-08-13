use std::{
    f32::consts::TAU,
    ops::Add,
    time::{self, Duration},
};

struct Sinusoid {
    freq: f32,
    amp: f32,
    offset: f32,
    func: fn(f32) -> f32,
}

type AirPressure = f32;

impl Sinusoid {
    fn new(freq: f32, amp: f32, offset: f32, func: fn(f32) -> f32) -> Sinusoid {
        Sinusoid {
            freq,
            amp,
            offset,
            func,
        }
    }

    fn period(&self) -> time::Duration {
        Duration::from_secs_f32(1.0 / self.freq)
    }

    fn evaluate(&self, times: Vec<time::Duration>) -> Vec<AirPressure> {
        times
            .iter()
            .map(|&t| {
                let phase = TAU * self.freq * t.as_secs_f32() + self.offset;
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
    let s = Sinusoid::new(440.0, 1.0, 0.0, f32::sin);
    assert_eq!(s.period(), Duration::from_secs_f32(1.0 / 440.0));
}

#[test]
// TODO more tests are needed, also check real values
fn sinusoid_signal_evaluate() {
    let s = Sinusoid::new(0.0, 0.0, 0.0, f32::sin);
    let times = vec![
        Duration::from_secs_f64(0.0),
        Duration::from_secs_f64(0.25),
        Duration::from_secs_f64(0.5),
        Duration::from_secs_f64(0.75),
    ];
    assert_eq!(s.evaluate(times), vec![0.0, 0.0, 0.0, 0.0])
}
