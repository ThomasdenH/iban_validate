use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iban::Iban;
use std::str::FromStr;

pub fn criterion_benchmark(c: &mut Criterion) {
    let iban_str = "DE44500105175407324931";
    c.bench_function(iban_str, |b| b.iter(|| Iban::from_str(black_box(iban_str))));
}

pub fn criterion_benchmark_with_spaces(c: &mut Criterion) {
    let iban_str = "LV80 BANK 0000 4351 9500 1";
    c.bench_function(iban_str, |b| b.iter(|| Iban::from_str(black_box(iban_str))));
}

pub fn display_benchmark(c: &mut Criterion) {
    let iban = Iban::from_str(black_box("DE44500105175407324931")).unwrap();
    c.bench_function("iban display", |b| b.iter(|| black_box(iban).to_string()));
}

pub fn display_with_spaces(c: &mut Criterion) {
    let iban = Iban::from_str(black_box("LV80 BANK 0000 4351 9500 1")).unwrap();
    c.bench_function("iban display #2", |b| {
        b.iter(|| black_box(iban).to_string())
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    criterion_benchmark_with_spaces,
    display_benchmark,
    display_with_spaces
);
criterion_main!(benches);
