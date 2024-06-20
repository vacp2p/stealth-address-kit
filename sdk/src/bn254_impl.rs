use crate::{define_curve_tests, stealth_addresses::StealthAddressOnCurve};
use ark_bn254::{Bn254, Fr, G1Projective};

impl StealthAddressOnCurve for Bn254 {
    type Projective = G1Projective;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(bn254, Bn254, Fr, G1Projective, 32, 32);
define_curve_tests!(Bn254);
