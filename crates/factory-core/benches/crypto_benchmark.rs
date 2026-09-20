use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ed25519_dalek::SigningKey;
use factory_core::security::nhi::{AgentSubject, VerifiableCredential};
use rand::rngs::OsRng;

fn bench_async_batch_signing(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    let subject = AgentSubject {
        id: "agent-bench-01".to_string(),
        roles: vec!["benchmark".to_string()],
        allowed_namespaces: vec!["factory-sandbox".to_string()],
    };

    c.bench_function("ed25519_async_batch_sign_10_vcs", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut batch = vec![
                    VerifiableCredential::new(
                        "vc-1".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-2".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-3".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-4".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-5".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-6".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-7".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-8".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-9".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                    VerifiableCredential::new(
                        "vc-10".to_string(),
                        "issuer".to_string(),
                        subject.clone(),
                    ),
                ];
                VerifiableCredential::sign_batch_async(&mut batch, &signing_key, "bench-key-01")
                    .await
                    .unwrap();
                black_box(batch);
            })
        });
    });
}

criterion_group!(benches, bench_async_batch_signing);
criterion_main!(benches);
