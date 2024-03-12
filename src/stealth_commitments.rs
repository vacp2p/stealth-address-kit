use ark_std::rand::rngs::OsRng;
use ark_std::UniformRand;
use std::fmt::Display;
use std::ops::{Add, Mul};

pub trait AffineWrapper {
    type Fq: ark_ff::PrimeField;
    fn new(x: Self::Fq, y: Self::Fq) -> Self;
}

pub trait RawFr {
    type Fr;
    fn as_u64(&self) -> u64;
}

pub trait StealthAddressOnCurve {
    type Projective: Display
        + Add<Output = Self::Projective>
        + Mul<Self::Fr, Output = Self::Projective>;
    type Affine: AffineWrapper;
    type Fr: Add<Self::Fr, Output = Self::Fr> + ark_ff::PrimeField + RawFr;

    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective;

    fn random_keypair() -> (Self::Fr, Self::Projective) {
        let private_key = Self::generate_random_fr();
        let public_key = Self::derive_public_key(&private_key);
        (private_key, public_key)
    }
    fn generate_random_fr() -> Self::Fr {
        let mut rng = OsRng;
        Self::Fr::rand(&mut rng)
    }
    fn hash_to_fr(input: &[u8]) -> Self::Fr;
    fn compute_shared_point(
        private_key: Self::Fr,
        public_key: Self::Projective,
    ) -> Self::Projective {
        public_key * private_key
    }

    fn generate_stealth_commitment(
        viewing_public_key: Self::Projective,
        spending_public_key: Self::Projective,
        ephemeral_private_key: Self::Fr,
    ) -> (Self::Projective, u64) {
        let q = Self::compute_shared_point(ephemeral_private_key, viewing_public_key);
        let inputs = q.to_string();
        let q_hashed = Self::hash_to_fr(inputs.as_bytes());

        let q_hashed_in_g1 = Self::derive_public_key(&q_hashed);
        let view_tag = q_hashed.as_u64();
        (q_hashed_in_g1 + spending_public_key, view_tag)
    }

    fn generate_stealth_private_key(
        ephemeral_public_key: Self::Projective,
        viewing_key: Self::Fr,
        spending_key: Self::Fr,
        expected_view_tag: u64,
    ) -> Option<Self::Fr> {
        let q_receiver = Self::compute_shared_point(viewing_key, ephemeral_public_key);

        let inputs_receiver = q_receiver.to_string();
        let q_receiver_hashed = Self::hash_to_fr(inputs_receiver.as_bytes());

        // Check if retrieved view tag matches the expected view tag
        let view_tag = q_receiver_hashed.as_u64();
        if view_tag == expected_view_tag {
            let stealth_private_key = spending_key + q_receiver_hashed;
            Some(stealth_private_key)
        } else {
            None
        }
    }
}
