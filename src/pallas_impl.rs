use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_pallas::{Affine, Fq, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};

pub struct Pallas;

impl StealthAddressOnCurve for Pallas {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Pallas);
