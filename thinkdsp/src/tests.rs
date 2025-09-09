//! Tests that any implementation must pass

// TODO more tests are needed, also check real values (maybe official rust lib for audio https://rust.audio/)
#[cfg(test)]
mod book {
    use crate::book::{CosSignal, Signal, SinSignal, Sinusoid, Wave};
    use std::f32::consts::PI;

    // Floating-point precision tolerance constants
    const EPSILON: f32 = 1e-6;
    const EPSILON_LARGE: f32 = 1e-2;

    #[test]
    fn signal_default_period() {
        struct TestSignal;
        impl Signal for TestSignal {
            fn evaluate(&self, _ts: &Vec<f32>) -> Vec<f32> {
                vec![]
            }
        }

        let signal = TestSignal;
        assert_eq!(signal.period(), 0.1);
    }

    #[test]
    fn signal_default_make_wave() {
        struct TestSignal;
        impl Signal for TestSignal {
            fn evaluate(&self, ts: &Vec<f32>) -> Vec<f32> {
                ts.iter().map(|&t| t * 2.0).collect()
            }
        }

        let signal = TestSignal;
        let wave = signal.make_wave(1.0, 0.0, 4);

        assert_eq!(wave.len(), 4);
        assert_eq!(wave.framerate, 4);
    }

    #[test]
    fn wave_new() {
        let ys = vec![1.0, 2.0, 3.0];
        let ts = vec![0.0, 0.1, 0.2];
        let framerate = 10;

        let wave = Wave::new(ys.clone(), ts.clone(), framerate);
        assert_eq!(wave.len(), 3);
        assert_eq!(wave.framerate, framerate);
    }

    #[test]
    fn wave_len() {
        let wave = Wave::new(vec![1.0, 2.0, 3.0, 4.0], vec![0.0, 0.1, 0.2, 0.3], 10);
        assert_eq!(wave.len(), 4);
    }

    #[test]
    fn wave_len_empty() {
        let wave = Wave::new(vec![], vec![], 10);
        assert_eq!(wave.len(), 0);
    }

    #[test]
    fn sinusoid_creation() {
        let s = Sinusoid::new(440.0, 1.0, 0.0, f32::sin);
        assert_eq!(s.period(), 1.0 / 440.0);
    }

    #[test]
    fn sinusoid_period() {
        let s = Sinusoid::new(100.0, 1.0, 0.0, f32::sin);
        assert_eq!(s.period(), 0.01);

        let s2 = Sinusoid::new(50.0, 1.0, 0.0, f32::sin);
        assert_eq!(s2.period(), 0.02);
    }

    #[test]
    fn sinusoid_evaluate_sine() {
        let s = Sinusoid::new(1.0, 1.0, 0.0, f32::sin);
        let times = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let result = s.evaluate(&times);

        // sin(2π * 1 * t) for t = 0, 0.25, 0.5, 0.75, 1.0
        // Expected: sin(0), sin(π/2), sin(π), sin(3π/2), sin(2π)
        // = 0, 1, 0, -1, 0
        assert!((result[0] - 0.0).abs() < EPSILON);
        assert!((result[1] - 1.0).abs() < EPSILON);
        assert!((result[2] - 0.0).abs() < EPSILON);
        assert!((result[3] - (-1.0)).abs() < EPSILON);
        assert!((result[4] - 0.0).abs() < EPSILON);
    }

    #[test]
    fn sinusoid_evaluate_cosine() {
        let s = Sinusoid::new(1.0, 1.0, 0.0, f32::cos);
        let times = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let result = s.evaluate(&times);

        // cos(2π * 1 * t) for t = 0, 0.25, 0.5, 0.75, 1.0
        // Expected: cos(0), cos(π/2), cos(π), cos(3π/2), cos(2π)
        // = 1, 0, -1, 0, 1
        assert!((result[0] - 1.0).abs() < EPSILON);
        assert!((result[1] - 0.0).abs() < EPSILON);
        assert!((result[2] - (-1.0)).abs() < EPSILON);
        assert!((result[3] - 0.0).abs() < EPSILON);
        assert!((result[4] - 1.0).abs() < EPSILON);
    }

    #[test]
    fn sinusoid_evaluate_with_amplitude() {
        let s = Sinusoid::new(1.0, 2.0, 0.0, f32::sin);
        let times = vec![0.25]; // sin(π/2) = 1, so 2 * 1 = 2
        let result = s.evaluate(&times);
        assert!((result[0] - 2.0).abs() < EPSILON);
    }

    #[test]
    fn sinusoid_evaluate_with_offset() {
        let s = Sinusoid::new(1.0, 1.0, PI / 2.0, f32::sin);
        let times = vec![0.0]; // sin(π/2) = 1
        let result = s.evaluate(&times);
        assert!((result[0] - 1.0).abs() < EPSILON);
    }

    #[test]
    fn sinusoid_make_wave() {
        let s = Sinusoid::new(440.0, 1.0, 0.0, f32::sin);
        let wave = s.make_wave(1.0, 0.0, 44100);
        assert_eq!(wave.len(), 44100);
    }

    #[test]
    fn sinusoid_make_wave_with_start() {
        let s = Sinusoid::new(1.0, 1.0, 0.0, f32::sin);
        let wave = s.make_wave(1.0, 0.5, 4);
        assert_eq!(wave.len(), 4);
        assert_eq!(wave.framerate, 4);
    }

    #[test]
    fn cos_signal_from_conversion() {
        let cos = CosSignal::new(100.0, 2.0, PI / 4.0);
        let sinusoid: Sinusoid = cos.into();

        // Test that the conversion preserves the properties
        assert_eq!(sinusoid.period(), 1.0 / 100.0);

        let times = vec![0.0];
        let result = sinusoid.evaluate(&times);
        // cos(π/4) = √2/2 ≈ 0.707
        assert!((result[0] - 2.0 * 0.707).abs() < EPSILON_LARGE);
    }

    #[test]
    fn sin_signal_from_conversion() {
        let sin = SinSignal::new(100.0, 2.0, PI / 4.0);
        let sinusoid: Sinusoid = sin.into();

        // Test that the conversion preserves the properties
        assert_eq!(sinusoid.period(), 1.0 / 100.0);

        let times = vec![0.0];
        let result = sinusoid.evaluate(&times);
        // sin(π/4) = √2/2 ≈ 0.707
        assert!((result[0] - 2.0 * 0.707).abs() < EPSILON_LARGE);
    }

    #[test]
    fn sum_signal_as_result_of_sinusoid_add() {
        let s1 = Sinusoid::new(440.0, 1.0, 0.0, f32::sin);
        let s2 = Sinusoid::new(880.0, 0.5, 0.0, f32::sin);

        let sum = s1 + s2;
        assert_eq!(sum.period(), 1.0 / 440.0); // max of the two periods
    }

    #[test]
    fn sum_signal_evaluate() {
        let s1 = Sinusoid::new(1.0, 1.0, 0.0, f32::sin);
        let s2 = Sinusoid::new(1.0, 2.0, 0.0, f32::cos);
        let sum = s1 + s2;

        let times = vec![0.0, 0.25, 0.5];
        let result = sum.evaluate(&times);

        // At t=0: sin(0) + 2*cos(0) = 0 + 2*1 = 2
        // At t=0.25: sin(π/2) + 2*cos(π/2) = 1 + 2*0 = 1
        // At t=0.5: sin(π) + 2*cos(π) = 0 + 2*(-1) = -2
        assert!((result[0] - 2.0).abs() < EPSILON);
        assert!((result[1] - 1.0).abs() < EPSILON);
        assert!((result[2] - (-2.0)).abs() < EPSILON);
    }

    #[test]
    fn sum_signal_make_wave() {
        let s1 = Sinusoid::new(1.0, 1.0, 0.0, f32::sin);
        let s2 = Sinusoid::new(1.0, 1.0, 0.0, f32::cos);
        let sum = s1 + s2;

        let wave = sum.make_wave(1.0, 0.0, 4);
        assert_eq!(wave.len(), 4);
        assert_eq!(wave.framerate, 4);
    }
}
