use crate::stealth_commitments::StealthAddressOnCurve;
use crate::{define_curve_ffi, define_curve_tests};
use ark_vesta::{Fr, Projective};

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
define_curve_ffi!(vesta, Vesta, Fr, Projective, 32, 33);
define_curve_tests!(Vesta);
