//! Tests that any implementation must pass

// TODO more tests are needed, also check real values (maybe official rust lib for audio https://rust.audio/)
#[cfg(test)]
mod book {
    use crate::book::{self, Signal};

    #[test]
    fn sinusoid_basic_functionality() {
        let s = book::Sinusoid::new(440.0, 1.0, 0.0, f32::sin);

        assert_eq!(s.period(), 1.0 / 440.0);

        // Test wave generation
        let wave = s.make_wave(1.0, 0.0, 44100);
        assert_eq!(wave.len(), 44100);
    }

    #[test]
    fn sinusoid_signal_evaluate() {
        let s = book::Sinusoid::new(0.0, 0.0, 0.0, f32::sin);
        let times = vec![0.0, 0.25, 0.5, 0.75];
        assert_eq!(s.evaluate(&times), vec![0.0, 0.0, 0.0, 0.0])
    }
}
