use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;

use ark_bls12_377::{Fr, G1Projective};

impl StealthAddressOnCurve for ark_bls12_377::Bls12_377 {
    type Projective = G1Projective;
    type Fr = Fr;
}

define_curve_tests!(ark_bls12_377::Bls12_377);
