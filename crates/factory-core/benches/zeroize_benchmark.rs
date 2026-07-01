use criterion::{black_box, criterion_group, criterion_main, Criterion};
use factory_core::security::JitToken;
use zeroize::Zeroize;

fn bench_zeroize_token(c: &mut Criterion) {
    c.bench_function("zeroize_jit_token", |b| {
        b.iter_batched(
            || {
                let mut token = JitToken {
                    token: String::from("abcdefghijklmnopqrstuvwxyz0123456789"),
                };
                // Ensure capacity is exactly what we need to avoid reallocation differences
                token.token.shrink_to_fit();
                token
            },
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
