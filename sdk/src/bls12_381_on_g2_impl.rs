use crate::stealth_addresses::StealthAddressOnCurve;
/// Implementation of the StealthAddressOnCurve trait for the Bls12_381 curve (G2).
use ark_bls12_381::G2Projective;

struct Bls12_381_G2;

impl StealthAddressOnCurve for Bls12_381_G2 {
    type Projective = G2Projective;
}
