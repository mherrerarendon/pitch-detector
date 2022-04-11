//! Pitch Detector is a pitch detection library written in Rust.
//!
//! ## Usage
//! Pitch Detector can be used in three different modes. You can use it to detect a frequency (pitch) from a signal...
//! ```rust
//! ```
//! or to detect the fundamental note being played in a signal, regardless of if it's out of tune...
//! ```rust
//! ```
//! or to detect a fundamental note being played in a signal that contains multiple notes. This is achieved by providing a hint to the detector about which note is being played...
//! ```rust
//! ```
//! Pitch Detector currently supports two different algorithms for pitch detection, but it can easily be expanded to support more algorithms. For more information on these algorithms, see the [pitch] module.
//!
//!
//!
pub mod core;
pub mod note;
pub mod pitch;
