# freq-detector

[![Build status](https://img.shields.io/github/workflow/status/mherrerarendon/freq-detector/Rust)](https://github.com/mherrerarendon/freq-detector)
[![codecov](https://img.shields.io/codecov/c/github/mherrerarendon/freq-detector)](https://codecov.io/gh/mherrerarendon/freq-detector)
<br/>

A frequency and pitch detector library written in Rust.

## Usage
```rust
use freq_detector::{detectors::raw_fft::RawFftDetector, FrequencyDetector};

const NUM_SAMPLES: usize = 16384;
const SAMPLE_RATE: f64 = 44100.0;
const FREQ: f64 = 440.0;

// Create the signal. 
let signal = (0..NUM_SAMPLES)
        .map(|r| (2.0 * std::f64::consts::PI * r as f64 * FREQ / SAMPLE_RATE).sin());

let mut detector = RawFftDetector;
let freq = detector.detect_frequency(signal, SAMPLE_RATE)?;

assert!(freq.approx_eq(FREQ, (0.02, 2)),);
```