use connx::encoding::base32;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("base32 encode hello", |b| {
        b.iter(|| base32::encode_str("hello"))
    });
    c.bench_function("base32 encode leasure.", |b| {
        b.iter(|| base32::encode_str("leasure."))
    });
    c.bench_function("base32 decode foobar", |b| {
        b.iter(|| base32::encode_str("MZXW6YTBOI======"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
