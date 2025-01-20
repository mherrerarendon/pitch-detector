use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pitch_detector::pitch::{HannedFftDetector, PitchDetector, PowerCepstrum};

pub fn test_signal(filename: &str) -> anyhow::Result<Vec<f64>> {
    let file_path = format!(
        "{}/test_data/audio_recordings/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    );
    let mut reader = hound::WavReader::open(file_path).unwrap();
    Ok(reader.samples::<i16>().map(|s| s.unwrap() as f64).collect())
}

fn criterion_benchmark(c: &mut Criterion) {
    let signal = test_signal("cello_open_a.wav").expect("Test file should exist");
    let mut hanned_detector = HannedFftDetector::default();
    let mut cepstrum_detector = PowerCepstrum::default();
    let sample_rate = 44100.;
    let mut group = c.benchmark_group("reduced_samples_group");
    group.significance_level(0.1).sample_size(60);
    group.bench_function("hanned fft", |b| {
        b.iter(|| {
            hanned_detector
                .detect_pitch(&signal, black_box(sample_rate))
                .unwrap()
        })
    });
    group.bench_function("power cepstrum", |b| {
        b.iter(|| {
            cepstrum_detector
                .detect_pitch(&signal, black_box(sample_rate))
                .unwrap()
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
