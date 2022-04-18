//! Pitch Detector is a pitch and note detection library written in Rust.
//!
//! ## Usage
//! Probably the most common use case is to detect the predominant frequency of a signal.
//! ```rust
//! use pitch_detector::{
//!     pitch::{hanned_fft::HannedFftDetector, PitchDetector},
//! };
//! # fn example_detect_frequency() -> anyhow::Result<()> {
//! # const NUM_SAMPLES: usize = 16384;
//! # const SAMPLE_RATE: f64 = 44100.0;
//!
//! let mut detector = HannedFftDetector::default();
//! let signal: Vec<f64> = Vec::new(); // Signal to analyze
//! let freq = detector
//!     .detect_pitch(&signal, SAMPLE_RATE)
//!     .ok_or(anyhow::anyhow!("Did not get pitch"))?;
//! # Ok(())
//! # }
//! ```
//! Another common use case is to detect the predominant note of a signal. This use case is similar to the first,
//! but the predominant note of the signal maps to a range of frequencies, which includes out-of-tune frequencies.
//! This use case is common for tuner applications, where the user would still want to know which note is being played,
//! even if it's out of tune. The return type of `detect_note` includes the offset in cents from the in-tune frequency,
//! and other useful information.
//! ```rust
//! use pitch_detector::{
//!     core::{utils::sine_wave_signal, NoteName},
//!     note::{detect_note},
//!     pitch::{hanned_fft::HannedFftDetector, PitchDetector},
//! };
//! # fn example_detect_note() -> anyhow::Result<()> {
//! # const NUM_SAMPLES: usize = 16384;
//! # const SAMPLE_RATE: f64 = 44100.0;
//!
//! let mut detector = HannedFftDetector::default();
//! let slightly_sharp_a = 448.;
//! let signal = sine_wave_signal(NUM_SAMPLES, slightly_sharp_a, SAMPLE_RATE);
//! let note = detect_note(
//!     &signal,
//!     &mut detector,
//!     SAMPLE_RATE,
//! )
//! .ok_or(anyhow::anyhow!("Did not get note"))?;
//!
//! assert_eq!(note.note_name, NoteName::A);
//! assert!(note.cents_offset > 0.);
//! #    Ok(())
//! # }
//! ```

//! The last use case is to detect a note with a hint. So far, the previous use cases have been about detecting
//! the predominant frequency or note. In this use case, we are providing the detector a hint so that it can
//! detect a frequency that might not be the predominant note. This is useful when there are multiple frequencies
//! in a signal (as there commonly are), but you want to know if the signal contains a specific note, and the
//! degree to which this specific note is in tune or not.
//! ```rust
//! use pitch_detector::{
//!     core::{utils::sine_wave_signal, NoteName},
//!     note::{hinted::HintedNoteDetector},
//!     pitch::{hanned_fft::HannedFftDetector, PitchDetector},
//! };
//! # fn example_hinted_note() -> anyhow::Result<()> {
//! # const NUM_SAMPLES: usize = 16384;
//! # const SAMPLE_RATE: f64 = 44100.0;
//!
//! let mut detector = HannedFftDetector::default();
//! let slightly_sharp_a = 448.;
//! let in_tune_c = 523.25;
//! let combined_signal: Vec<f64> = Vec::new();
//! let note = detector
//!     .detect_note_with_hint(
//!         NoteName::A,
//!         &combined_signal,
//!         SAMPLE_RATE,
//!     )
//!     .ok_or(anyhow::anyhow!("Did not get note"))?;
//!
//! assert_eq!(note.note_name, NoteName::A);
//! assert!(note.cents_offset > 0.);
//! #    Ok(())
//! # }
//! ```
//! Check out the `examples` directory for more.
//! Pitch Detector currently supports two different algorithms for pitch detection, but it can easily be
//! expanded to support more algorithms. For more information on these algorithms, see the [pitch] module.
//!
//!
//!
pub mod core;
pub mod note;
pub mod pitch;

#[cfg(feature = "plot")]
pub mod plot;
