# erc-5564-rs

Uses the [arkworks-rs](https://github.com/arkworks-rs/curves) suite of libraries, and utilities from [rln](https://github.com/vacp2p/zerokit)

## Existing Implementations

1. `ark_bn254`
2. `ark_bls_12_381`
3. `ark_bls_12_377`
4. `secp256k1`

## Usage

```rust
use erc_5564_rs::{StealthAddressOnCurve};
use ark_bn254::Bn254; // or ark_bls_12_381::Bls12_381 or ark_bls_12_377::Bls12_377, or erc_5564_rs::Secp256k1

fn main() {
    let (spending_key, spending_public_key) = Bn254::random_keypair();
    let (viewing_key, viewing_public_key) = Bn254::random_keypair();

    // generate ephemeral keypair
    let (ephemeral_private_key, ephemeral_public_key) = Bn254::random_keypair();

    let (stealth_commitment, view_tag) = Bn254::generate_stealth_commitment(viewing_public_key, spending_public_key, ephemeral_private_key);

    let stealth_private_key_opt = Bn254::generate_stealth_private_key(ephemeral_public_key, viewing_key, spending_key, view_tag);

    if stealth_private_key_opt.is_none() {
        panic!("View tags did not match");
    }

    let derived_commitment = Bn254::derive_public_key(stealth_private_key_opt.unwrap());
    assert_eq!(derived_commitment, stealth_commitment);
}
```

## Building and Testing

1. Building
   `cargo build --release --features <bn254/bls12_381/bls12_377/secp256k1>`

2. Testing
   `cargo test --release --features <bn254/bls12_381/bls12_377/secp256k1>`

## FFI Api

The exposed FFI API supports all curves, prefixed by the curve name. Ensure that the correct feature is enabled when building the library.

## Precompiled Libraries

Check out the nightly releases.

## Attribution

- The original circuits for rln are located [here](https://github.com/Rate-Limting-Nullifier/circom-rln), by the PSE group
- Inspired by the [erc-5564](https://eips.ethereum.org/EIPS/eip-5564) eip and the [poc](https://github.com/nerolation/EIP-Stealth-Address-ERC/blob/main/minimal_poc.ipynb) by Nerolation.
