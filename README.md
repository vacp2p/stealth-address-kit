# erc-5564-bn254

Uses the [arkworks-rs](https://github.com/arkworks-rs/curves) suite of libraries, and utilities from [rln](https://github.com/vacp2p/zerokit)

## Usage

```rust
use erc_5564_bn254::{random_keypair, generate_stealth_commitment, generate_stealth_private_key};

fn main() {
    let (spending_key, spending_public_key) = random_keypair();
    let (viewing_key, viewing_public_key) = random_keypair();

// generate ephemeral keypair
    let (ephemeral_private_key, ephemeral_public_key) = random_keypair();

    let (stealth_commitment, view_tag) = generate_stealth_commitment(viewing_public_key, spending_public_key, ephemeral_private_key);

    let stealth_private_key_opt = generate_stealth_private_key(ephemeral_public_key, viewing_key, spending_key, view_tag);

    if stealth_private_key_opt.is_none() {
        panic!("View tags did not match");
    }

    let derived_commitment = derive_public_key(stealth_private_key_opt.unwrap());
    assert_eq!(derived_commitment, stealth_commitment);
}
```

## Building and Testing

1. Building
    `cargo build --release`

2. Testing
    `cargo test --release`

## Attribution

Inspired by the [erc-5564](https://eips.ethereum.org/EIPS/eip-5564) eip and the [poc](https://github.com/nerolation/EIP-Stealth-Address-ERC/blob/main/minimal_poc.ipynb) by Nerolation.