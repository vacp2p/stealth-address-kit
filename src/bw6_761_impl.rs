use crate::stealth_addresses::StealthAddressOnCurve;
use crate::{define_curve_ffi, define_curve_tests};

use ark_bw6_761::{Fr, G1Projective, BW6_761};

impl StealthAddressOnCurve for BW6_761 {
    type Projective = G1Projective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
define_curve_ffi!(bw6_761, BW6_761, Fr, G1Projective, 48, 96);
define_curve_tests!(BW6_761);
