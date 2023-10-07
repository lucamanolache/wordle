use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use std::fs;
use wordle::calc_probs;

#[derive(Debug, Deserialize)]
struct Line {
    probability: f32,
    word: String,
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("calc_probs");
    g.sample_size(15);

    let mut rdr =
        csv::Reader::from_path("freqs.csv").expect("Should have been able to read the file");
    let contents: Vec<([u8; 5], f32)> = rdr
        .deserialize()
        .map(|res| -> ([u8; 5], f32) {
            let record: Line = res.unwrap();

            (
                record.word.as_bytes().try_into().unwrap(),
                record.probability,
            )
        })
        .collect();
    g.bench_function("calc_probs", |b| b.iter(|| calc_probs(&contents)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
