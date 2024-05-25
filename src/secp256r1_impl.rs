use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_secp256r1::{Affine, Fq, Fr, Projective};

pub struct Secp256r1;

impl StealthAddressOnCurve for Secp256r1 {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Secp256r1);
