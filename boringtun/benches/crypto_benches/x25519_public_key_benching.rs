use criterion::Criterion;
use rand_core::OsRng;

pub fn bench_x25519_public_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("x25519_public_key");

    group.sample_size(1000);

    group.bench_function("x25519_public_key_dalek", |b| {
        b.iter(|| {
            let secret_key = x25519_dalek::StaticSecret::random_from_rng(OsRng);
            let public_key = x25519_dalek::PublicKey::from(&secret_key);

            (secret_key, public_key)
        });
    });

    group.finish();
}
