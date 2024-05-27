use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;
use ark_pallas::{Fr, Projective};

pub struct Pallas;

impl StealthAddressOnCurve for Pallas {
    type Projective = Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(pallas, Pallas, Fr, Projective, 32, 33);
define_curve_tests!(Pallas);
