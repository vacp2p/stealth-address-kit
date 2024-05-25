use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_bls12_381::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bls12_381::{Fq, Fr, G1Affine, G1Projective};

impl StealthAddressOnCurve for ark_bls12_381::Bls12_381 {
    type Projective = G1Projective;
    type Fr = Fr;
}

define_curve_tests!(ark_bls12_381::Bls12_381);
