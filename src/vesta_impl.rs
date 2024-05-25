use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_vesta::{Fr, Projective};

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Vesta);
