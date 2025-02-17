use fitting::gaussian::fit;

use super::{error::PitchError, FftPoint};

pub fn sine_wave_signal(num_samples: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
    (0..num_samples)
        .map(|r| (2.0 * std::f64::consts::PI * r as f64 * freq / sample_rate).sin())
        .collect()
}

pub fn mixed_wave_signal(num_samples: usize, freqs: Vec<f64>, sample_rate: f64) -> Vec<f64> {
    let mut signal = vec![0.0; num_samples];
    freqs.iter().for_each(|f| {
        let s = sine_wave_signal(num_samples, *f, sample_rate);
        signal.iter_mut().zip(s.iter()).for_each(|(a, b)| *a += *b);
    });
    signal
}

pub fn audio_buffer_to_samples(byte_buffer: &[u8]) -> Box<dyn Iterator<Item = i16> + '_> {
    Box::new(
        byte_buffer
            .chunks_exact(2)
            .map(|a| i16::from_ne_bytes([a[0], a[1]])),
    )
}
pub fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Box<dyn Iterator<Item = f64> + '_> {
    Box::new(audio_buffer_to_samples(byte_buffer).map(|x| x as f64))
}

/// Fits the curve to which fft_point_x belongs to and returns the peak point
pub fn interpolated_peak_at(spectrum: &[f64], fft_point_x: usize) -> Result<FftPoint, PitchError> {
    let mut idx = fft_point_x;
    let peak_begin_idx = loop {
        if idx == 0 {
            break idx;
        }
        if spectrum[idx] < spectrum[idx - 1] || spectrum[idx - 1] <= 0. {
            break idx;
        }
        idx -= 1;
    };
    idx = fft_point_x;
    let peak_end_idx_incl = loop {
        if idx == spectrum.len() - 1 {
            break idx;
        }
        if spectrum[idx] < spectrum[idx + 1] || spectrum[idx + 1] <= 0. {
            break idx;
        }
        idx += 1;
    };
    let y_vals: Vec<f64> = spectrum[peak_begin_idx..=peak_end_idx_incl].to_vec();
    let x_vals: Vec<f64> = (peak_begin_idx..=peak_end_idx_incl)
        .map(|i| i as f64)
        .collect();

    assert_eq!(
        y_vals.len(),
        x_vals.len(),
        "Expected y_vals and x_vals to equal in length"
    );
    match x_vals.len() {
        0 => Err(PitchError::IncorrectParameters(
            "Expected at least one x value".to_string(),
        )),
        1 => Ok(FftPoint {
            x: x_vals[0],
            y: y_vals[0],
        }),
        2 => {
            if y_vals[0] > y_vals[1] {
                Ok(FftPoint {
                    x: x_vals[0],
                    y: y_vals[0],
                })
            } else {
                Ok(FftPoint {
                    x: x_vals[1],
                    y: y_vals[1],
                })
            }
        }
        _ => {
            let (mu, _, a) = fit(x_vals.into(), y_vals.into())
                .map_err(|e| PitchError::UnexpectedError(e.to_string()))?;
            Ok(FftPoint { x: mu, y: a })
        }
    }
}
