use crate::{define_curve_tests, stealth_addresses::StealthAddressOnCurve};
/// Implementation of the StealthAddressOnCurve trait for the Bls12_377 curve.
use ark_bls12_377::{Bls12_377, Fr, G1Projective};

impl StealthAddressOnCurve for Bls12_377 {
    type Projective = G1Projective;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(bls12_377, Bls12_377, Fr, G1Projective, 32, 48);
define_curve_tests!(Bls12_377);
