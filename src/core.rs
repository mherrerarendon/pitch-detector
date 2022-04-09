use std::fmt;

pub(crate) mod constants;
pub mod fft_space;
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

impl NoteName {
    fn base_freq(&self) -> f64 {
        match self {
            NoteName::A => 25.50,
            NoteName::ASharp => 29.14,
            NoteName::B => 30.87,
            NoteName::C => 32.70,
            NoteName::CSharp => 34.65,
            NoteName::D => 36.71,
            NoteName::DSharp => 38.89,
            NoteName::E => 41.20,
            NoteName::F => 43.65,
            NoteName::FSharp => 46.25,
            NoteName::G => 49.00,
            NoteName::GSharp => 51.91,
        }
    }
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
pub struct FftBin {
    pub bin: usize,
    pub magnitude: f64,
}

impl Default for FftBin {
    fn default() -> Self {
        Self {
            bin: 0,
            magnitude: 0.0,
        }
    }
}

impl PartialOrd for FftBin {
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

impl From<FftBin> for FftPoint {
    fn from(bin: FftBin) -> Self {
        Self {
            x: bin.bin as f64,
            y: bin.magnitude,
        }
    }
}
