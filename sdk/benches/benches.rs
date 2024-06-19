/// This benchmark is used to benchmark all implementations which are enabled by the `--features` flag.
use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;
use stealth_address_kit::StealthAddressOnCurve;

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "secp256k1")]
    {
        use stealth_address_kit::Secp256k1;
        define_curve_benchmarks!(Secp256k1, c);
    }
    #[cfg(feature = "secp256r1")]
    {
        use stealth_address_kit::Secp256r1;
        define_curve_benchmarks!(Secp256r1, c);
    }
    #[cfg(feature = "bn254")]
    {
        use ark_bn254::Bn254;
        define_curve_benchmarks!(Bn254, c);
    }
    #[cfg(feature = "bls12_381")]
    {
        use ark_bls12_381::Bls12_381;
        define_curve_benchmarks!(Bls12_381, c);
    }
    #[cfg(feature = "bls12_377")]
    {
        use ark_bls12_377::Bls12_377;
        define_curve_benchmarks!(Bls12_377, c);
    }
    #[cfg(feature = "pallas")]
    {
        use stealth_address_kit::Pallas;
        define_curve_benchmarks!(Pallas, c);
    }
    #[cfg(feature = "vesta")]
    {
        use stealth_address_kit::Vesta;
        define_curve_benchmarks!(Vesta, c);
    }
    #[cfg(feature = "baby_jub_jub")]
    {
        use stealth_address_kit::BabyJubJub;
        define_curve_benchmarks!(BabyJubJub, c);
    }
    #[cfg(feature = "bw6_761")]
    {
        use ark_bw6_761::BW6_761;
        define_curve_benchmarks!(BW6_761, c);
    }
}

#[macro_export]
macro_rules! define_curve_benchmarks {
    ($Curve:ty, $c:ty) => {
        paste! {
            let [<$Curve:lower _random_keypair>] = <$Curve>::random_keypair();
            let mut group = $c.benchmark_group(stringify!([<$Curve:lower>]));

            group.bench_function("derive_public_key", |b| {
                b.iter(|| {
                    let _ = <$Curve>::derive_public_key(&[<$Curve:lower _random_keypair>].0);
                })
            });

            group.bench_function("random_keypair", |b| {
                b.iter(|| {
                    let _ = <$Curve>::random_keypair();
                })
            });

            group.bench_function("generate_random_fr", |b| {
                b.iter(|| {
                    let _ = <$Curve>::generate_random_fr();
                })
            });

            let random_u8_slice = [0u8; 32];

            group.bench_function("hash_to_fr", |b| {
                b.iter(|| {
                    let _ = <$Curve>::hash_to_fr(&random_u8_slice);
                })
            });

            group.bench_function("compute_shared_point", |b| {
                b.iter(|| {
                    let _ = <$Curve>::compute_shared_point([<$Curve:lower _random_keypair>].0, [<$Curve:lower _random_keypair>].1);
                })
            });

            let [<$Curve:lower _random_keypair_2>] = <$Curve>::random_keypair();

            group.bench_function("generate_stealth_address", |b| {
                b.iter(|| {
                    let _ = <$Curve>::generate_stealth_address(
                        [<$Curve:lower _random_keypair>].1,
                        [<$Curve:lower _random_keypair_2>].1,
                        [<$Curve:lower _random_keypair>].0,
                    );
                })
            });

            group.bench_function("generate_stealth_private_key", |b| {
                b.iter(|| {
                    let _ = <$Curve>::generate_stealth_private_key(
                        [<$Curve:lower _random_keypair>].1,
                        [<$Curve:lower _random_keypair>].0,
                        [<$Curve:lower _random_keypair_2>].0,
                        0,
                    );
                })
            });

            group.finish();
        }
    };
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
