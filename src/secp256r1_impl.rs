use crate::stealth_addresses::StealthAddressOnCurve;
use crate::{define_curve_ffi, define_curve_tests};
use ark_secp256r1::{Fr, Projective};

pub struct Secp256r1;

impl StealthAddressOnCurve for Secp256r1 {
    type Projective = Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
define_curve_ffi!(secp256r1, Secp256r1, Fr, Projective, 32, 33);
define_curve_tests!(Secp256r1);
