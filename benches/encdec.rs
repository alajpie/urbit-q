use criterion::{black_box, criterion_group, criterion_main, Criterion};
use urbit_q::*;

fn enc1k(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..1024).map(|_| rand::random()).collect();
    c.bench_function("encode 1k", |b| b.iter(|| encode(black_box(&bytes))));
}

fn enc8(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..8).map(|_| rand::random()).collect();
    c.bench_function("encode 8", |b| b.iter(|| encode(black_box(&bytes))));
}

fn dec1k(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..1024).map(|_| rand::random()).collect();
    let string = encode(&bytes);
    c.bench_function("decode 1k", |b| b.iter(|| decode(black_box(&string))));
}

fn dec8(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..8).map(|_| rand::random()).collect();
    let string = encode(&bytes);
    c.bench_function("decode 8", |b| b.iter(|| decode(black_box(&string))));
}

criterion_group!(benches, enc1k, enc8, dec1k, dec8);
criterion_main!(benches);
