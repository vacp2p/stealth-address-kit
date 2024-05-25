use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, StealthAddressOnCurve};
use ark_vesta::{Affine, Fq, Fr, Projective, G_GENERATOR_X, G_GENERATOR_Y};

#[allow(non_camel_case_types)]
pub struct VestaAffine(Affine);
impl AffineWrapper for VestaAffine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        VestaAffine(Affine::new(x, y))
    }

    fn get_generator_x() -> Self::Fq {
        G_GENERATOR_X
    }

    fn get_generator_y() -> Self::Fq {
        G_GENERATOR_Y
    }
}

impl From<VestaAffine> for Projective {
    fn from(value: VestaAffine) -> Self {
        Projective::from(value.0)
    }
}

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
    type Affine = VestaAffine;
    type Fr = Fr;
}

define_curve_tests!(Vesta);
