use fitting::gaussian::fit;

use crate::frequency::FftPoint;

pub fn sine_wave_signal(num_samples: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
    (0..num_samples)
        .map(|r| (2.0 * std::f64::consts::PI * r as f64 * freq / sample_rate).sin())
        .collect()
}

pub fn audio_buffer_to_samples(byte_buffer: &[u8]) -> Vec<i16> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .collect()
}
pub fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
    audio_buffer_to_samples(byte_buffer)
        .into_iter()
        .map(|x| x as f64)
        .collect()
}

pub fn interpolated_peak_at(spectrum: &[f64], fft_point_x: usize) -> Option<FftPoint> {
    // TODO: indexes are off here
    let mut idx = fft_point_x;
    let peak_begin_idx = loop {
        if idx == 0 {
            break idx;
        }
        if spectrum[idx] < spectrum[idx - 1] {
            break idx;
        }
        idx -= 1;
    };
    idx = fft_point_x;
    let peak_end_idx_incl = loop {
        if idx == spectrum.len() - 1 {
            break idx;
        }
        if spectrum[idx] < spectrum[idx + 1] {
            break idx;
        }
        idx += 1;
    };
    let y_vals: Vec<f64> = spectrum[peak_begin_idx..=peak_end_idx_incl]
        .iter()
        .cloned()
        .collect();
    let x_vals: Vec<f64> = (peak_begin_idx..=peak_end_idx_incl)
        .map(|i| i as f64)
        .collect();

    assert_eq!(y_vals.len(), x_vals.len(), "Sanity check failed");
    match x_vals.len() {
        0 => None,
        1 => Some(FftPoint {
            x: x_vals[0],
            y: y_vals[0],
        }),
        2 => {
            if y_vals[0] > y_vals[1] {
                Some(FftPoint {
                    x: x_vals[0],
                    y: y_vals[0],
                })
            } else {
                Some(FftPoint {
                    x: x_vals[1],
                    y: y_vals[1],
                })
            }
        }
        _ => {
            let (mu, _, a) = fit(x_vals.into(), y_vals.into()).ok()?;
            Some(FftPoint { x: mu, y: a })
        }
    }
}
