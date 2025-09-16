use std::time::Duration;

use rodio::{ChannelCount, SampleRate, Source};

use crate::book::Wave as BookWave;

/// Represents a discrete-time waveform.
pub struct Wave {
    inner: BookWave,
    idx: usize, // internal playback cursor
}

impl Iterator for Wave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.ys.len() {
            return None;
        }
        let y = self.inner.ys[self.idx];
        self.idx += 1;
        Some(y)
    }
}

impl Source for Wave {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        1
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.framerate as u32 // TODO refactor me
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl From<BookWave> for Wave {
    fn from(wave: BookWave) -> Self {
        Wave {
            inner: wave,
            idx: 0,
        }
    }
}
