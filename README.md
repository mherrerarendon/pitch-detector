# pitch-detector

[![Build status](https://img.shields.io/github/workflow/status/mherrerarendon/freq-detector/Rust)](https://github.com/mherrerarendon/freq-detector)
[![codecov](https://img.shields.io/codecov/c/github/mherrerarendon/freq-detector)](https://codecov.io/gh/mherrerarendon/freq-detector)
<br/>

A pitch and note detector library written in Rust.

## Usage
```rust
use pitch_detector::{
    core::{NoteName},
    note::{detect_note},
    pitch::{hanned_fft::HannedFftDetector, PitchDetector},
};
use float_cmp::ApproxEq; // Used only to compare floats
...

const NUM_SAMPLES: usize = 16384;
const SAMPLE_RATE: f64 = 44100.;
const FREQ: f64 = 440.;
const MAX_FREQ: f64 = 1046.50; // C6
const MIN_FREQ: f64 = 32.7; // C1

// Create the signal. 
let signal = (0..NUM_SAMPLES)
        .map(|r| (2. * std::f64::consts::PI * r as f64 * FREQ / SAMPLE_RATE).sin());

let mut detector = HannedFftDetector::default();

// Detect predominant frequency of signal
let freq = detector
    .detect_pitch(&signal, SAMPLE_RATE, Some(MIN_FREQ..MAX_FREQ))?
assert!(freq.approx_eq(FREQ, (0.02, 2)),);

// Detect predominant note of signal
let note = detect_note(
        &signal,
        &mut detector,
        SAMPLE_RATE,
        Some(MIN_FREQ..MAX_FREQ),
    )?;
assert_eq!(note.note_name, NoteName::A); 

```

## License
Licensed under the [MIT license](https://github.com/mherrerarendon/pitch-detector/blob/main/LICENSE-MIT)