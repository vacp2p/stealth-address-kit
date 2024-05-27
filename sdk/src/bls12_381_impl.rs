use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;

use ark_bls12_381::{Bls12_381, Fr, G1Projective};

impl StealthAddressOnCurve for Bls12_381 {
    type Projective = G1Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(bls12_381, Bls12_381, Fr, G1Projective, 32, 48);
define_curve_tests!(Bls12_381);
