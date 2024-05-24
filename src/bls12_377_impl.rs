use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, RawFr, StealthAddressOnCurve};
use ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bls12_377::{Fq, Fr, G1Affine, G1Projective};
use ark_ff::PrimeField;
use tiny_keccak::{Hasher, Keccak};

#[allow(non_camel_case_types)]
pub struct Bls12_377_G1Affine(G1Affine);
impl AffineWrapper for Bls12_377_G1Affine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        Bls12_377_G1Affine(G1Affine::new(x, y))
    }
}

impl From<Bls12_377_G1Affine> for G1Projective {
    fn from(value: Bls12_377_G1Affine) -> Self {
        G1Projective::from(value.0)
    }
}

impl RawFr for Fr {
    type Fr = Fr;
    fn as_u64(&self) -> u64 {
        self.0 .0[0]
    }
}

impl StealthAddressOnCurve for ark_bls12_377::Bls12_377 {
    type Projective = G1Projective;
    type Affine = Bls12_377_G1Affine;
    type Fr = Fr;

    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective {
        let g1_generator_affine = Self::Affine::new(G1_GENERATOR_X, G1_GENERATOR_Y);
        (Self::Projective::from(g1_generator_affine)) * *private_key
    }

    fn hash_to_fr(input: &[u8]) -> Self::Fr {
        let mut hash = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(input);
        hasher.finalize(&mut hash);

        // We export the hash as a field element
        Self::Fr::from_le_bytes_mod_order(hash.as_slice())
    }
}

define_curve_tests!(ark_bls12_377::Bls12_377);
