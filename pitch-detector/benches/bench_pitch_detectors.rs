use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pitch_detector::{
    core::test_utils::test_signal,
    pitch::{HannedFftDetector, PitchDetector, PowerCepstrum},
};

fn criterion_benchmark(c: &mut Criterion) {
    let signal = test_signal("cello_open_a.json").expect("Test file should exist");
    let mut hanned_detector = HannedFftDetector::default();
    let mut cepstrum_detector = PowerCepstrum::default();
    let sample_rate = 44000.;
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
