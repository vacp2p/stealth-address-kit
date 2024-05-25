use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_pallas::{Fr, Projective};

pub struct Pallas;

impl StealthAddressOnCurve for Pallas {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Pallas);
