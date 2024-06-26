use crate::{define_curve_tests, stealth_addresses::StealthAddressOnCurve};
/// Implementation of the StealthAddressOnCurve trait for the Secp256r1 curve.
use ark_secp256r1::{Fr, Projective};

pub struct Secp256r1;

impl StealthAddressOnCurve for Secp256r1 {
    type Projective = Projective;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(secp256r1, Secp256r1, Fr, Projective, 32, 33);
define_curve_tests!(Secp256r1);
