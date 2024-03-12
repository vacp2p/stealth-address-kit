use crate::stealth_commitments::{AffineWrapper, RawFr, StealthAddressOnCurve};
use ark_bn254::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bn254::{Fq, Fr, G1Affine, G1Projective};
use rln::hashers::{hash_to_field, poseidon_hash};

impl AffineWrapper for G1Affine {
    type Fq = Fq;
    fn new(x: Self::Fq, y: Self::Fq) -> Self {
        G1Affine::new(x, y)
    }
}

impl RawFr for Fr {
    type Fr = Fr;
    fn as_u64(&self) -> u64 {
        self.0 .0[0]
    }
}

impl StealthAddressOnCurve for ark_bn254::Bn254 {
    type Projective = G1Projective;
    type Affine = G1Affine;
    type Fr = Fr;

    fn derive_public_key(private_key: &Self::Fr) -> Self::Projective {
        let g1_generator_affine = Self::Affine::new(G1_GENERATOR_X, G1_GENERATOR_Y);
        (Self::Projective::from(g1_generator_affine)) * *private_key
    }

    fn hash_to_fr(input: &[u8]) -> Self::Fr {
        poseidon_hash(&[hash_to_field(input)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;
    use ark_std::rand::thread_rng;
    use ark_std::UniformRand;
    use color_eyre::{Report, Result};
    use rln::public::RLN;
    use rln::utils::fr_to_bytes_le;
    use serde_json::json;
    use std::io::Cursor;

    type Curve = ark_bn254::Bn254;

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

    // this can only be tested for bn254 since that is the curve supported by RLN
    #[test]
    fn apply_stealth_membership_from_one_tree_to_another() -> Result<()> {
        let test_tree_height = 20;
        let resources = Cursor::new(json!({"resources_folder": "tree_height_20"}).to_string());
        let mut rln = RLN::new(test_tree_height, resources.clone())?;

        let alice_leaf = Fr::rand(&mut thread_rng());
        let (alice_known_spending_sk, alice_known_spending_pk) = Curve::random_keypair();
        let alice_leaf_buffer = Cursor::new(fr_to_bytes_le(&alice_leaf));
        rln.set_leaf(0, alice_leaf_buffer)?;

        // now the application sees that a user has been inserted into the tree
        let mut rln_app_tree = RLN::new(test_tree_height, resources)?;
        // the application generates a stealth commitment for alice
        let (ephemeral_private_key, ephemeral_public_key) = Curve::random_keypair();
        let (alice_stealth_commitment, view_tag) = Curve::generate_stealth_commitment(
            alice_known_spending_pk,
            alice_known_spending_pk,
            ephemeral_private_key,
        );

        let parts = [alice_stealth_commitment.x, alice_stealth_commitment.y];
        let fr_parts = parts.map(|x| Fr::from(x.0));
        let alice_stealth_commitment_buffer =
            Cursor::new(fr_to_bytes_le(&poseidon_hash(&fr_parts)));
        rln_app_tree.set_leaf(0, alice_stealth_commitment_buffer)?;

        // now alice's stealth commitment has been inserted into the tree, but alice has not
        // yet derived the secret for it -
        let alice_stealth_private_key_opt = Curve::generate_stealth_private_key(
            ephemeral_public_key,
            alice_known_spending_sk,
            alice_known_spending_sk,
            view_tag,
        );
        if alice_stealth_private_key_opt.is_none() {
            return Err(Report::msg("Invalid view tag"));
        }
        let alice_stealth_private_key = alice_stealth_private_key_opt.unwrap();

        assert_eq!(
            Curve::derive_public_key(&alice_stealth_private_key),
            alice_stealth_commitment
        );

        // now alice may generate valid rln proofs for the rln app tree, using a commitment
        // derived from her commitment on the other tree
        Ok(())
    }
}
