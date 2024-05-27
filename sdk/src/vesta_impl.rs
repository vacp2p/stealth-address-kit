use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;
use ark_vesta::{Fr, Projective};

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(vesta, Vesta, Fr, Projective, 32, 33);
define_curve_tests!(Vesta);
