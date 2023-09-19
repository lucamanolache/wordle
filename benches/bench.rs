use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use wordle::calc_probs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("calc_probs");
    g.sample_size(15);

    let fp = "allowed_words.txt";
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();
    g.bench_function("calc_probs", |b| {
        b.iter(|| calc_probs(&contents, &contents))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);