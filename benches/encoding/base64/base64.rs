use criterion::{criterion_group, criterion_main, Criterion};
use connx::encoding::base64;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("base64 encode hello", |b| b.iter(|| base64::encode_str("hello")));
    c.bench_function("base64 encode leasure.", |b| b.iter(|| base64::encode_str("leasure.")));
    c.bench_function("base64 decode leasure.", |b| b.iter(|| base64::encode_str("bGVhc3VyZS4=")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
