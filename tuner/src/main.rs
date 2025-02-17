mod note_renderers;

use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, StreamConfig};
use dasp_sample::ToSample;
use note_renderers::cmd_line::CmdLineNoteRenderer;
use note_renderers::simple_command_line::SimpleCommandLineRenderer;
use note_renderers::NoteRenderer;
use pitch_detector::note::detect_note_in_range;
use pitch_detector::pitch::{Cepstrum2, PowerCepstrum};
use tokio::select;
use tokio_util::sync::CancellationToken;

#[tracing::instrument(skip_all)]
fn write_input_data<T, Renderer>(input: &[T], renderer: Arc<Renderer>)
where
    T: Sample + ToSample<f64>,
    Renderer: NoteRenderer,
{
    const SAMPLE_RATE: f64 = 44100.0;
    const MAX_FREQ: f64 = 1046.50; // C6
    const MIN_FREQ: f64 = 32.7; // C1

    let mut detector = PowerCepstrum::new_with_defaults().with_sigmas(0.5);
    // let mut detector = Cepstrum2;

    // TODO: maybe have the detector work in terms of the Sample trait instead of a specific type
    // to avoid another allocation
    let signal = input
        .iter()
        .map(|s| s.to_sample::<f64>())
        .collect::<Vec<f64>>();

    // TODO: handle unwraps
    match detect_note_in_range(&signal, &mut detector, SAMPLE_RATE, MIN_FREQ..MAX_FREQ) {
        Ok(note) => renderer.render_note(note).unwrap(),
        Err(e) => renderer.render_no_note(e).unwrap(),
    }
}

async fn listen_audio<Renderer>(
    config: StreamConfig,
    device: Device,
    renderer: Arc<Renderer>,
) -> anyhow::Result<()>
where
    Renderer: NoteRenderer + Send + Sync + 'static,
{
    // A flag to indicate that recording is in progress.
    println!("Begin recording...");
    renderer.initialize()?;

    // Run the input stream on a separate thread.

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let renderer_clone = renderer.clone();
    let stream = device.build_input_stream(
        &config,
        move |data, _: &_| write_input_data::<f32, _>(data, renderer_clone.clone()),
        err_fn,
        None,
    )?;

    stream.play()?;

    let token = CancellationToken::new();
    let cloned_token = token.clone();

    select! {
        _ = cloned_token.cancelled() => {
            let _ = renderer.tear_down();
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = host
        .default_input_device()
        .expect("failed to find input device");

    let config = device.default_input_config()?;
    println!("supported buffer size: {:?}", config.buffer_size());

    // Set the buffer size to the maximum supported value. The larger the buffer, the more accurate the
    // pitch detection algorithm
    let config = StreamConfig {
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        // The 'buffer_size' is actually specified in the Stream creation
        // as the number of frames (i.e., samples per channel)
        buffer_size: cpal::BufferSize::Fixed(4096), // Fixed buffer size
    };

    println!("Input device: {}", device.name()?);

    println!("Input config: {:?}", config);

    let cmd_line_renderer = Arc::new(SimpleCommandLineRenderer);
    listen_audio(config, device, cmd_line_renderer).await?;

    Ok(())
}
