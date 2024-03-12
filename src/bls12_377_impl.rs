use crate::stealth_commitments::{AffineWrapper, RawFr, StealthAddressOnCurve};
use ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bls12_377::{Fq, Fr, G1Affine, G1Projective};
use ark_ff::PrimeField;
use tiny_keccak::{Hasher, Keccak};

#[allow(non_camel_case_types)]
pub struct Bls12_377_G1Affine(G1Affine);
impl AffineWrapper for Bls12_377_G1Affine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        Bls12_377_G1Affine(G1Affine::new(x, y))
    }
}

impl From<Bls12_377_G1Affine> for G1Projective {
    fn from(value: Bls12_377_G1Affine) -> Self {
        G1Projective::from(value.0)
    }
}

impl RawFr for Fr {
    type Fr = Fr;
    fn as_u64(&self) -> u64 {
        self.0 .0[0]
    }
}

impl StealthAddressOnCurve for ark_bls12_377::Bls12_377 {
    type Projective = G1Projective;
    type Affine = Bls12_377_G1Affine;
    type Fr = Fr;

    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective {
        let g1_generator_affine = Self::Affine::new(G1_GENERATOR_X, G1_GENERATOR_Y);
        (Self::Projective::from(g1_generator_affine)) * *private_key
    }

    fn hash_to_fr(input: &[u8]) -> Self::Fr {
        let mut hash = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(input);
        hasher.finalize(&mut hash);

        // We export the hash as a field element
        Self::Fr::from_le_bytes_mod_order(hash.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;

    type Curve = ark_bls12_377::Bls12_377;

    #[test]
    fn test_random_keypair() {
        let (key, pub_key) = Curve::random_keypair();
        // Check the derived key matches the one generated from original key
        assert_eq!(Curve::derive_public_key(&key), pub_key);
    }

    #[test]
    fn test_hash_to_fr() {
        // Test that hash_to_fr(input_1) != hash_to_fr(input_2) when input_1 != input_2
        let input_1 = b"input_1";
        let input_2 = b"input_2";
        assert_ne!(Curve::hash_to_fr(input_1), Curve::hash_to_fr(input_2));
    }

    #[test]
    fn test_compute_shared_point() {
        // In a multiple participant scenario, any participant's public key
        // combined with any other participant's private key should arrive at the same shared key
        let (key1, pub_key1) = Curve::random_keypair();
        let (key2, pub_key2) = Curve::random_keypair();

        let shared1 = Curve::compute_shared_point(key1, pub_key2);
        let shared2 = Curve::compute_shared_point(key2, pub_key1);

        // Convert Projective to Affine for equality comparison
        let shared1_affine = shared1.into_affine();
        let shared2_affine = shared2.into_affine();

        assert_eq!(shared1_affine.x, shared2_affine.x);
        assert_eq!(shared1_affine.y, shared2_affine.y);
    }

    #[test]
    fn test_stealth_commitment_generation() {
        let (spending_key, spending_public_key) = Curve::random_keypair();
        let (viewing_key, viewing_public_key) = Curve::random_keypair();

        // generate ephemeral keypair
        let (ephemeral_private_key, ephemeral_public_key) = Curve::random_keypair();

        let (stealth_commitment, view_tag) = Curve::generate_stealth_commitment(
            viewing_public_key,
            spending_public_key,
            ephemeral_private_key,
        );

        let stealth_private_key_opt = Curve::generate_stealth_private_key(
            ephemeral_public_key,
            viewing_key,
            spending_key,
            view_tag,
        );

        if stealth_private_key_opt.is_none() {
            panic!("View tags did not match");
        }

        let derived_commitment = Curve::derive_public_key(&stealth_private_key_opt.unwrap());
        assert_eq!(derived_commitment, stealth_commitment);
    }
}
