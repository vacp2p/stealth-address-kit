#[macro_export]
macro_rules! define_curve_tests {
    ($Curve:ty) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use ark_ec::CurveGroup;

            #[test]
            fn random_keypair_happy_path() {
                let (key, pub_key) = <$Curve>::random_keypair();
                // Check the derived key matches the one generated from original key
                assert_eq!(<$Curve>::derive_public_key(&key), pub_key);
            }

            #[test]
            fn hash_to_fr_happy_path() {
                // Test that hash_to_fr(input_1) != hash_to_fr(input_2) when input_1 != input_2
                let input_1 = b"input_1";
                let input_2 = b"input_2";
                assert_ne!(<$Curve>::hash_to_fr(input_1), <$Curve>::hash_to_fr(input_2));
            }

            #[test]
            fn compute_shared_point_happy_path() {
                // In a multiple participant scenario, any participant's public key
                // combined with any other participant's private key should arrive at the same shared key
                let (key1, pub_key1) = <$Curve>::random_keypair();
                let (key2, pub_key2) = <$Curve>::random_keypair();

                let shared1 = <$Curve>::compute_shared_point(key1, pub_key2);
                let shared2 = <$Curve>::compute_shared_point(key2, pub_key1);

                // Convert Projective to Affine for equality comparison
                let shared1_affine = shared1.into_affine();
                let shared2_affine = shared2.into_affine();

                assert_eq!(shared1_affine.x, shared2_affine.x);
                assert_eq!(shared1_affine.y, shared2_affine.y);
            }

            #[test]
            fn generate_stealth_address_happy_path() {
                let (spending_key, spending_public_key) = <$Curve>::random_keypair();
                let (viewing_key, viewing_public_key) = <$Curve>::random_keypair();

                // generate ephemeral keypair
                let (ephemeral_private_key, ephemeral_public_key) = <$Curve>::random_keypair();

                let (stealth_address, view_tag) = <$Curve>::generate_stealth_address(
                    viewing_public_key,
                    spending_public_key,
                    ephemeral_private_key,
                );

                let stealth_private_key_opt = <$Curve>::generate_stealth_private_key(
                    ephemeral_public_key,
                    viewing_key,
                    spending_key,
                    view_tag,
                );

                if stealth_private_key_opt.is_none() {
                    panic!("View tags did not match");
                }

                let derived_address =
                    <$Curve>::derive_public_key(&stealth_private_key_opt.unwrap());
                assert_eq!(derived_address, stealth_address);
            }
        }
    };
}
