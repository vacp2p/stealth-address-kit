use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, StealthAddressOnCurve};
use ark_bls12_381::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bls12_381::{Fq, Fr, G1Affine, G1Projective};

#[allow(non_camel_case_types)]
pub struct Bls12_381_G1Affine(G1Affine);
impl AffineWrapper for Bls12_381_G1Affine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        Bls12_381_G1Affine(G1Affine::new(x, y))
    }

    fn get_generator_x() -> Self::Fq {
        G1_GENERATOR_X
    }

    fn get_generator_y() -> Self::Fq {
        G1_GENERATOR_Y
    }
}

impl From<Bls12_381_G1Affine> for G1Projective {
    fn from(value: Bls12_381_G1Affine) -> Self {
        G1Projective::from(value.0)
    }
}

impl StealthAddressOnCurve for ark_bls12_381::Bls12_381 {
    type Projective = G1Projective;
    type Affine = Bls12_381_G1Affine;
    type Fr = Fr;
}

define_curve_tests!(ark_bls12_381::Bls12_381);
