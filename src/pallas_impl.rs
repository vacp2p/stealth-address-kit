use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, StealthAddressOnCurve};
use ark_pallas::{Affine, Fq, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};

#[allow(non_camel_case_types)]
pub struct PallasAffine(Affine);
impl AffineWrapper for PallasAffine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        PallasAffine(Affine::new(x, y))
    }

    fn get_generator_x() -> Self::Fq {
        G_GENERATOR_X
    }

    fn get_generator_y() -> Self::Fq {
        G_GENERATOR_Y
    }
}

impl From<PallasAffine> for Projective {
    fn from(value: PallasAffine) -> Self {
        Projective::from(value.0)
    }
}

pub struct Pallas;

impl StealthAddressOnCurve for Pallas {
    type Projective = Projective;
    type Affine = PallasAffine;
    type Fr = Fr;
}

define_curve_tests!(Pallas);
