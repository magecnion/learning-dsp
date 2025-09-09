//! https://github.com/AllenDowney/ThinkDSP/blob/f1cc15de31f658d5df287332a30659fb16eb41d5/code/chap01.ipynb

use thinkdsp::book::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let cosine = CosSignal::new(440.0, 1.0, 0.0);
    Signal::plot(&Sinusoid::from(cosine), 11025);
}
