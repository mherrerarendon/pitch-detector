mod note_renderers;

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use dasp_sample::ToSample;
use pitch_detector::{core::NoteName, note::detect_note_in_range, pitch::HannedFftDetector};

fn write_input_data<T>(input: &[T])
where
    T: Sample + ToSample<f64>,
{
    const SAMPLE_RATE: f64 = 44100.0;
    const MAX_FREQ: f64 = 1046.50; // C6
    const MIN_FREQ: f64 = 32.7; // C1
    let mut detector = HannedFftDetector::default();

    // TODO: maybe have the detector work in terms of the Sample trait instead of a specific type
    let signal = input
        .iter()
        .map(|s| s.to_sample::<f64>())
        .collect::<Vec<f64>>();

    // TODO: handle unwrap
    let note = detect_note_in_range(&signal, &mut detector, SAMPLE_RATE, MIN_FREQ..MAX_FREQ)
        .ok_or(anyhow::anyhow!("Did not get note"))
        .unwrap();
}

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = host
        .default_input_device()
        .expect("failed to find input device");

    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32>(data),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(stream);
    Ok(())
}
