use ark_ec::CurveGroup;
use ark_ff::{Fp, FpConfig, PrimeField};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::rngs::OsRng;
use ark_std::UniformRand;
use std::fmt::Display;
use std::ops::{Add, Mul};

use tiny_keccak::{Hasher, Keccak};

pub trait AffineWrapper {
    type Fq: ark_ff::PrimeField;
    fn new(x: Self::Fq, y: Self::Fq) -> Self;
    fn get_generator_x() -> Self::Fq;
    fn get_generator_y() -> Self::Fq;
}

pub trait HasViewTag {
    fn get_view_tag(&self) -> u64;
}

// Implement HasViewTag for any Fp type
impl<P: FpConfig<N>, const N: usize> HasViewTag for Fp<P, N> {
    fn get_view_tag(&self) -> u64 {
        self.0 .0[0]
    }
}

pub trait ToBytesFromProjective {
    fn to_bytes(&self) -> Vec<u8>;
}

// Implement ToBytesFromProjective for any ProjectiveCurve
impl<G: CurveGroup> ToBytesFromProjective for G
where
    G::Affine: CanonicalSerialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        let affine = self.into_affine();
        let mut bytes = Vec::new();
        affine.serialize_uncompressed(&mut bytes).unwrap();
        bytes
    }
}

pub trait StealthAddressOnCurve {
    type Projective: Display
        + Add<Output = Self::Projective>
        + Mul<Self::Fr, Output = Self::Projective>
        + From<Self::Affine>
        + ark_ec::CurveGroup;
    type Affine: AffineWrapper;
    type Fr: Add<Self::Fr, Output = Self::Fr> + ark_ff::PrimeField + HasViewTag;
    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective {
        let generator_affine = Self::Affine::new(
            Self::Affine::get_generator_x(),
            Self::Affine::get_generator_y(),
        );
        (Self::Projective::from(generator_affine)) * *private_key
    }

    fn random_keypair() -> (Self::Fr, Self::Projective) {
        let private_key = Self::generate_random_fr();
        let public_key = Self::derive_public_key(&private_key);
        (private_key, public_key)
    }
    fn generate_random_fr() -> Self::Fr {
        let mut rng = OsRng;
        Self::Fr::rand(&mut rng)
    }
    fn hash_to_fr(input: &[u8]) -> Self::Fr {
        let mut hash = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(input);
        hasher.finalize(&mut hash);

        // We export the hash as a field element
        Self::Fr::from_le_bytes_mod_order(hash.as_slice())
    }
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
        let q_hashed = Self::hash_to_fr(&q.to_bytes());
        let q_hashed_in_g1 = Self::derive_public_key(&q_hashed);
        let view_tag = q_hashed.get_view_tag();
        (q_hashed_in_g1 + spending_public_key, view_tag)
    }

    fn generate_stealth_private_key(
        ephemeral_public_key: Self::Projective,
        viewing_key: Self::Fr,
        spending_key: Self::Fr,
        expected_view_tag: u64,
    ) -> Option<Self::Fr> {
        let q_receiver = Self::compute_shared_point(viewing_key, ephemeral_public_key);

        let q_receiver_hashed = Self::hash_to_fr(&q_receiver.to_bytes());

        // Check if retrieved view tag matches the expected view tag
        let view_tag = q_receiver_hashed.get_view_tag();
        if view_tag == expected_view_tag {
            let stealth_private_key = spending_key + q_receiver_hashed;
            Some(stealth_private_key)
        } else {
            None
        }
    }
}
