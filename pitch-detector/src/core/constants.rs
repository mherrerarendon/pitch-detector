pub const MAX_FREQ: f64 = 1046.50; // C6
pub const MIN_FREQ: f64 = 32.7; // C1
pub const MIN_ZERO_CROSSING_RATE: f64 = 350.; // Hz
pub const A4_FREQ: f64 = 440.0;
pub const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// Noticable pitch difference starts at around 10-25 cents
pub const MAX_CENTS_OFFSET: f64 = 10.0;

pub const NUM_CENTS_BETWEEN_NOTES: f64 = 100.;
