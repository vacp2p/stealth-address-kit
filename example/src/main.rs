use stealth_address_kit::Secp256r1;
use stealth_address_kit::StealthAddressOnCurve;

type Curve = Secp256r1;

fn main() {
    let (spending_key, spending_public_key) = Curve::random_keypair();
    let (viewing_key, viewing_public_key) = Curve::random_keypair();

    // generate ephemeral keypair
    let (ephemeral_private_key, ephemeral_public_key) = Curve::random_keypair();

    let (stealth_address, view_tag) = Curve::generate_stealth_address(
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

    let derived_stealth_address = Curve::derive_public_key(&stealth_private_key_opt.unwrap());
    assert_eq!(derived_stealth_address, stealth_address);
}
