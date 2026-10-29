use criterion::{criterion_group, criterion_main, Criterion};
use factory_core::security::JitToken;
use std::hint::black_box;
use zeroize::Zeroize;

fn bench_zeroize_token(c: &mut Criterion) {
    c.bench_function("zeroize_jit_token", |b| {
        b.iter_batched(
            || JitToken::new("abcdefghijklmnopqrstuvwxyz0123456789"),
            |mut token| {
                token.zeroize();
                black_box(token);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_zeroize_token);
criterion_main!(benches);
