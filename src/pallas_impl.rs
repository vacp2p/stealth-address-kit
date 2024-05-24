use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, RawFr, StealthAddressOnCurve};
use ark_ff::PrimeField;
use ark_pallas::{Affine, Fq, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};
use tiny_keccak::{Hasher, Keccak};

#[allow(non_camel_case_types)]
pub struct PallasAffine(Affine);
impl AffineWrapper for PallasAffine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        PallasAffine(Affine::new(x, y))
    }
}

impl From<PallasAffine> for Projective {
    fn from(value: PallasAffine) -> Self {
        Projective::from(value.0)
    }
}

impl RawFr for Fr {
    type Fr = Fr;
    fn as_u64(&self) -> u64 {
        self.0 .0[0]
    }
}

pub struct Pallas;

impl StealthAddressOnCurve for Pallas {
    type Projective = Projective;
    type Affine = PallasAffine;
    type Fr = Fr;

    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective {
        let generator_affine = Self::Affine::new(G_GENERATOR_X, G_GENERATOR_Y);
        (Self::Projective::from(generator_affine)) * *private_key
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

define_curve_tests!(Pallas);
