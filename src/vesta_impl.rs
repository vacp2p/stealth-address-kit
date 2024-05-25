use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_vesta::{Affine, Fq, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
    type Fr = Fr;
}

define_curve_tests!(Vesta);
