use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_secp256k1::{Fr, Projective};

pub struct Secp256k1;

impl StealthAddressOnCurve for Secp256k1 {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Secp256k1);
