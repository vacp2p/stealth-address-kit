use crate::define_curve_tests;
use crate::stealth_addresses::StealthAddressOnCurve;

use ark_ed_on_bn254::{EdwardsProjective, Fr};

pub struct BabyJubJub;

impl StealthAddressOnCurve for BabyJubJub {
    type Projective = EdwardsProjective;
    type Fr = Fr;
}

#[cfg(feature = "ffi")]
use crate::define_curve_ffi;
#[cfg(feature = "ffi")]
define_curve_ffi!(baby_jub_jub, BabyJubJub, Fr, EdwardsProjective, 32, 32);
define_curve_tests!(BabyJubJub);
