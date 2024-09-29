fn read_callback(stream: &mut soundio::InStreamReader) {
    let mut frames_left = stream.frame_count_max();

    // libsoundio reads samples in chunks, so we need to loop until there's nothing to read.
    loop {
        if let Err(e) = stream.begin_read(frames_left) {
            println!("Error reading from stream: {}", e);
            return;
        }
        for f in 0..stream.frame_count() {
            for c in 0..stream.channel_count() {
                // In reality you shouldn't write to disk in the callback, but have some buffer instead.
                todo!()
            }
        }

        frames_left -= stream.frame_count();
        if frames_left <= 0 {
            break;
        }

        stream.end_read();
    }
}
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    // TODO: Probe which channels/sample rates are available.
    let channels = 2;
    let sample_rate = 44100;

    let spec = hound::WavSpec {
        channels: channels,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Try to open the output file.

    let mut ctx = soundio::Context::new();
    ctx.set_app_name("Recorder");
    ctx.connect()?;

    println!("Current backend: {:?}", ctx.current_backend());

    // We have to flush events so we can scan devices.
    ctx.flush_events();
    // I guess these are always signed little endian?
    let soundio_format = soundio::Format::S16LE;

    let default_layout = soundio::ChannelLayout::get_default(channels as _);
    println!(
        "Default layout for {} channel(s): {:?}",
        channels, default_layout
    );

    let input_dev = ctx
        .default_input_device()
        .map_err(|_| "Error getting default input device".to_string())?;

    println!(
        "Default input device: {} {}",
        input_dev.name(),
        if input_dev.is_raw() { "raw" } else { "cooked" }
    );

    let mut recorder = WavRecorder { writer: writer };

    println!("Opening default input stream");
    let mut input_stream = input_dev.open_instream(
        sample_rate as _,
        soundio_format,
        default_layout,
        0.1,
        |x| recorder.read_callback(x),
        None::<fn()>,
        None::<fn(soundio::Error)>,
    )?;

    println!("Starting stream");
    input_stream.start()?;

    // Wait for the user to press a key.
    println!("Press enter to stop recording");
    let stdin = io::stdin();
    let input = &mut String::new();
    let _ = stdin.read_line(input);

    Ok(())
}
