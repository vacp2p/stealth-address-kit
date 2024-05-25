use crate::define_curve_tests;
use crate::stealth_commitments::{AffineWrapper, StealthAddressOnCurve};
use ark_secp256k1::{Affine, Fq, Fr, Projective};
use ark_secp256k1::{G_GENERATOR_X, G_GENERATOR_Y};

#[allow(non_camel_case_types)]
pub struct Secp256k1_Affine(Affine);
impl AffineWrapper for Secp256k1_Affine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        Secp256k1_Affine(Affine::new(x, y))
    }

    fn get_generator_x() -> Self::Fq {
        G_GENERATOR_X
    }

    fn get_generator_y() -> Self::Fq {
        G_GENERATOR_Y
    }
}

impl From<Secp256k1_Affine> for Projective {
    fn from(value: Secp256k1_Affine) -> Self {
        Projective::from(value.0)
    }
}

pub struct Secp256k1;

impl StealthAddressOnCurve for Secp256k1 {
    type Projective = Projective;
    type Affine = Secp256k1_Affine;
    type Fr = Fr;
}

define_curve_tests!(Secp256k1);
