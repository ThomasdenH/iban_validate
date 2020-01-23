use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iban::Iban;
use std::str::FromStr;

pub fn criterion_benchmark(c: &mut Criterion) {
    let iban_str = "DE44500105175407324931";
    c.bench_function(iban_str, |b| {
        b.iter(|| Iban::from_str(black_box(iban_str))?)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
