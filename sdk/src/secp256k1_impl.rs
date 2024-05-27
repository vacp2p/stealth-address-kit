use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;
use ark_secp256k1::{Fr, Projective};

pub struct Secp256k1;

impl StealthAddressOnCurve for Secp256k1 {
    type Projective = Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(secp256k1, Secp256k1, Fr, Projective, 32, 33);
define_curve_tests!(Secp256k1);