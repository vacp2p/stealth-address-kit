use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;

use ark_bls12_381::{Fr, G1Projective};

impl StealthAddressOnCurve for ark_bls12_381::Bls12_381 {
    type Projective = G1Projective;
    type Fr = Fr;
}

define_curve_tests!(ark_bls12_381::Bls12_381);
