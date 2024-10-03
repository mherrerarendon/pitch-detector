use std::fmt;

pub mod constants;
pub mod error;
pub mod fft_space;
pub mod into_frequency_domain;
pub mod utils;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

impl From<&str> for NoteName {
    fn from(s: &str) -> Self {
        match s {
            "A" => NoteName::A,
            "A#" => NoteName::ASharp,
            "B" => NoteName::B,
            "C" => NoteName::C,
            "C#" => NoteName::CSharp,
            "D" => NoteName::D,
            "D#" => NoteName::DSharp,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "F#" => NoteName::FSharp,
            "G" => NoteName::G,
            "G#" => NoteName::GSharp,
            _ => panic!("Invalid pitch"),
        }
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NoteName::A => write!(f, "A"),
            NoteName::ASharp => write!(f, "A#"),
            NoteName::B => write!(f, "B"),
            NoteName::C => write!(f, "C"),
            NoteName::CSharp => write!(f, "C#"),
            NoteName::D => write!(f, "D"),
            NoteName::DSharp => write!(f, "D#"),
            NoteName::E => write!(f, "E"),
            NoteName::F => write!(f, "F"),
            NoteName::FSharp => write!(f, "F#"),
            NoteName::G => write!(f, "G"),
            NoteName::GSharp => write!(f, "G#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyBin {
    pub bin: usize,
    pub magnitude: f64,
}

impl Default for FrequencyBin {
    fn default() -> Self {
        Self {
            bin: 0,
            magnitude: 0.0,
        }
    }
}

impl PartialOrd for FrequencyBin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude.partial_cmp(&other.magnitude)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FftPoint {
    pub x: f64,
    pub y: f64,
}

impl Default for FftPoint {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl From<FrequencyBin> for FftPoint {
    fn from(bin: FrequencyBin) -> Self {
        Self {
            x: bin.bin as f64,
            y: bin.magnitude,
        }
    }
}
