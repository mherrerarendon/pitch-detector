# pitch-detector

[![Build status](https://img.shields.io/github/workflow/status/mherrerarendon/freq-detector/Rust)](https://github.com/mherrerarendon/freq-detector)
[![codecov](https://img.shields.io/codecov/c/github/mherrerarendon/freq-detector)](https://codecov.io/gh/mherrerarendon/freq-detector)
<br/>

A pitch and note detector library written in Rust.

## Usage
Probably the most common use case is to detect the predominant frequency of a signal. 
```rust
use pitch_detector::{
    pitch::{hanned_fft::HannedFftDetector, PitchDetector},
};

let sample_rate = 44100.
let signal: Vec<f64> = ...;

let mut detector = HannedFftDetector::default();
let freq = detector.detect_pitch(&signal, sample_rate)?;
```
Another common use case is to detect the predominant note of a signal. This use case is similar to the first, but the predominant note of the signal maps to a range of frequencies, which includes out-of-tune frequencies. This use case is common for tuner applications, where the user would still want to know which note is being played, even if it's out of tune. The return type of `detect_note` includes the offset in cents from the in-tune frequency, and other useful information.
```rust
use pitch_detector::{
    pitch::{hanned_fft::HannedFftDetector, PitchDetector},
    note::{detect_note},
};

let sample_rate = 44100.
let signal: Vec<f64> = ...;

let mut detector = HannedFftDetector::default();
let note = detect_note(
        &signal,
        &mut detector,
        sample_rate,
    )?;
```

The last use case is to detect a note with a hint. So far, the previous use cases have been about detecting the predominant frequency or note. In this use case, we are providing the detector a hint so that it can detect a frequency that might not be the predominant note. This is useful when there are multiple frequencies in a signal (as there commonly are), but you want to know if the signal contains a specific note, and the degree to which this specific note is in tune or not.
```rust
let sample_rate = 44100.
let mixed_signal: Vec<f64> = ... // mixed_signal contains multiple overlapping frequencies

let note = detector
        .detect_note_with_hint(
            NoteName::A,
            &mixed_signal,
            sample_rate,
        )
        .ok_or(anyhow::anyhow!("Did not detect hinted note in signal"))?;
assert_eq!(note.note_name, NoteName::A);
assert!(note.cents_offset > 0.);
```
Check out the `examples` directory for more.

## License
Licensed under the [MIT license](https://github.com/mherrerarendon/pitch-detector/blob/main/LICENSE-MIT)