use crate::{define_curve_tests, stealth_addresses::StealthAddressOnCurve};
/// Implementation of the StealthAddressOnCurve trait for the Vesta curve.
use ark_vesta::{Fr, Projective};

pub struct Vesta;

impl StealthAddressOnCurve for Vesta {
    type Projective = Projective;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(vesta, Vesta, Fr, Projective, 32, 33);
define_curve_tests!(Vesta);
