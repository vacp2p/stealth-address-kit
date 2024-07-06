use crate::{define_curve_tests, stealth_addresses::StealthAddressOnCurve};
/// Implementation of the StealthAddressOnCurve trait for the Bls12_381 curve (G2).
use ark_bls12_381::G2Projective;

struct Bls12_381_G2;

impl StealthAddressOnCurve for Bls12_381_G2 {
    type Projective = G2Projective;
}

// define the tests for the curve
define_curve_tests!(Bls12_381_G2);
// run the tests with `cargo test -p stealth_address_kit --no-default-features --features bls12_381 bls12_381_on_g2_impl`
