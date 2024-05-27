use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;

use ark_bls12_377::{Bls12_377, Fr, G1Projective};

impl StealthAddressOnCurve for Bls12_377 {
    type Projective = G1Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(bls12_377, Bls12_377, Fr, G1Projective, 32, 48);
define_curve_tests!(ark_bls12_377::Bls12_377);
