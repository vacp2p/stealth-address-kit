use stealth_address_kit::Secp256k1 as Curve;
use stealth_address_kit::StealthAddressOnCurve;

fn print_discriminator() {
    println!("{}", "+".repeat(100));
}

fn main() {
    let (spending_key, spending_public_key) = Curve::random_keypair();
    let (viewing_key, viewing_public_key) = Curve::random_keypair();

    print_discriminator();
    println!("BOB PRE-COMPUTATION");
    print_discriminator();

    println!("Spending Key: {}", &spending_key.to_string());
    println!("Spending Public Key: {}", &spending_public_key.to_string());
    println!("Viewing Key: {}", &viewing_key.to_string());
    println!("Viewing Public Key: {}", &viewing_public_key.to_string());

    print_discriminator();

    // generate ephemeral keypair
    let (ephemeral_private_key, ephemeral_public_key) = Curve::random_keypair();

    print_discriminator();
    println!("ALICE COMPUTATION");
    print_discriminator();

    println!(
        "Ephemeral Private Key: {}",
        &ephemeral_private_key.to_string()
    );
    println!(
        "Ephemeral Public Key: {}",
        &ephemeral_public_key.to_string()
    );

    let (stealth_public_key, view_tag) = Curve::generate_stealth_address(
        viewing_public_key,
        spending_public_key,
        ephemeral_private_key,
    );

    println!("Stealth Public Key: {}", &stealth_public_key.to_string());
    println!("View Tag: {}", &view_tag.to_string());

    print_discriminator();

    print_discriminator();
    println!("BOB COMPUTATION AFTER RECEIVING BROADCASTED KEY MATERIAL");
    print_discriminator();

    let stealth_private_key_opt = Curve::generate_stealth_private_key(
        ephemeral_public_key,
        viewing_key,
        spending_key,
        view_tag,
    );

    if let Some(stealth_private_key) = stealth_private_key_opt {
        let derived_stealth_public_key = Curve::derive_public_key(&stealth_private_key);

        println!(
            "Derived Stealth Public Key: {}",
            &derived_stealth_public_key.to_string()
        );
        println!("Stealth Private Key: {}", &stealth_private_key.to_string());

        assert_eq!(derived_stealth_public_key, stealth_public_key);

        print_discriminator();
    } else {
        panic!("View tags did not match");
    };
}
