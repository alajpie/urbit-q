use criterion::{black_box, criterion_group, criterion_main, Criterion};
use urbit_q::*;

fn enc(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..1024).map(|_| rand::random()).collect();
    c.bench_function("encode 1k", |b| {
        b.iter(|| encode(black_box(&bytes)))
    });
}

fn dec(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..1024).map(|_| rand::random()).collect();
    let string = encode(&bytes);
    c.bench_function("decode 1k", |b| {
        b.iter(|| decode(black_box(&string)))
    });
}

criterion_group!(benches, enc, dec);
criterion_main!(benches);
