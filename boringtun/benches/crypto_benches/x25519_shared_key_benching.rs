use criterion::{BatchSize, Criterion};
use rand_core::OsRng;

pub fn bench_x25519_shared_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("x25519_shared_key");

    group.sample_size(1000);

    group.bench_function("x25519_shared_key_dalek", |b| {
        let public_key =
            x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::random_from_rng(OsRng));

        b.iter_batched(
            || x25519_dalek::StaticSecret::random_from_rng(OsRng),
            |secret_key| secret_key.diffie_hellman(&public_key),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}
