use crate::define_curve_tests;
use crate::stealth_commitments::StealthAddressOnCurve;
use ark_bn254::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};
use ark_bn254::{Fq, Fr, G1Affine, G1Projective};
use rln::hashers::{hash_to_field, poseidon_hash};

impl StealthAddressOnCurve for ark_bn254::Bn254 {
    type Projective = G1Projective;
    type Fr = Fr;

    fn hash_to_fr(input: &[u8]) -> Self::Fr {
        poseidon_hash(&[hash_to_field(input)])
    }
}

define_curve_tests!(ark_bn254::Bn254);

#[cfg(test)]
mod rln_tests {
    use super::*;
    use ark_std::rand::thread_rng;
    use ark_std::UniformRand;
    use color_eyre::{Report, Result};
    use rln::public::RLN;
    use rln::utils::fr_to_bytes_le;
    use serde_json::json;
    use std::io::Cursor;

    type Curve = ark_bn254::Bn254;

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
