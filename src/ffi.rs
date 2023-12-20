use std::ops::Add;
use std::str::FromStr;
use crate::stealth_commitments::{derive_public_key, generate_random_fr, generate_stealth_commitment, generate_stealth_private_key, random_keypair};
use ark_bn254::{Fq, Fr, G1Projective, G1Affine};
use ark_ff::{BigInt, BigInteger, Field, PrimeField, ToConstraintField};
use num_traits::{Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[repr(C)]
#[derive(Debug)]
pub struct CFr([u8; 32]);

impl Add for CFr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CFr::from(Fr::from(self).add(Fr::from(rhs)))
    }
}

impl Zero for CFr {
    fn zero() -> Self {
        CFr::from(Fr::from(0))
    }

    fn is_zero(&self) -> bool {
        Fr::is_zero(&Fr::from(self))
    }
}

impl From<Fr> for CFr {
    fn from(value: Fr) -> Self {
        let mut buf = Vec::new();
        value.serialize_compressed(&mut buf).unwrap();
        let mut res = [0u8; 32];
        res.copy_from_slice(&buf);
        CFr(res)
    }
}

impl From<CFr> for Fr {
    fn from(value: CFr) -> Self {
        Fr::deserialize_compressed(value.0.as_slice()).unwrap()
    }
}

impl From<&CFr> for Fr {
    fn from(value: &CFr) -> Self {
        Fr::deserialize_compressed(value.0.as_slice()).unwrap()
    }
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct CG1Projective([u8; 32]);

impl From<G1Projective> for CG1Projective {
    fn from(value: G1Projective) -> Self {
        let mut buf = Vec::new();
        value.serialize_compressed(&mut buf).unwrap();
        let mut result = [0u8; 32];
        result.copy_from_slice(&buf);
        CG1Projective(result)
    }
}
impl From<CG1Projective> for G1Projective {
    fn from(value: CG1Projective) -> Self {
        G1Projective::deserialize_compressed(value.0.as_slice()).expect("TODO: panic message")
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CKeyPair {
    private_key: CFr,
    public_key: CG1Projective
}

#[repr(C)]
#[derive(Debug)]
pub struct CStealthCommitment {
    stealth_commitment: CG1Projective,
    view_tag: u64
}

impl From<(G1Projective, u64)> for CStealthCommitment {
    fn from(value: (G1Projective, u64)) -> Self {
        CStealthCommitment {
            stealth_commitment: CG1Projective::from(value.0),
            view_tag: value.1
        }
    }
}

impl Into<(G1Projective, u64)> for CStealthCommitment {
    fn into(self) -> (G1Projective, u64) {
        (self.stealth_commitment.into(), self.view_tag)
    }
}

#[no_mangle]
pub extern "C" fn ffi_generate_random_fr() -> CFr {
    CFr::from(generate_random_fr())
}

#[no_mangle]
pub extern "C" fn ffi_derive_public_key(private_key: CFr) -> CG1Projective {
    CG1Projective::from(derive_public_key(private_key.into()))
}


pub extern "C" fn ffi_random_keypair() -> CKeyPair {
    let (private_key, public_key) = random_keypair();
    CKeyPair {
        private_key: CFr::from(private_key),
        public_key: CG1Projective::from(public_key)
    }
}

pub extern "C" fn ffi_generate_stealth_commitment(viewing_public_key: CG1Projective, spending_public_key: CG1Projective, ephemeral_private_key: CFr) -> CStealthCommitment {
    CStealthCommitment::from(generate_stealth_commitment(viewing_public_key.into(), spending_public_key.into(), ephemeral_private_key.into()))
}

pub extern "C" fn ffi_generate_stealth_private_key(ephemeral_public_key: CG1Projective, spending_key: CFr, viewing_key: CFr, view_tag: u64) -> CFr {
    match generate_stealth_private_key(ephemeral_public_key.into(), spending_key.into(), viewing_key.into(), view_tag) {
        Some(v) => CFr::from(v),
        None => CFr::zero()
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::ToConstraintField;
    use super::*;
    use crate::stealth_commitments::{derive_public_key};
    use ark_ec::{AffineRepr, CurveGroup};


    #[test]
    fn test_ffi_generate_random_fr() {
        let _ = ffi_generate_random_fr();
    }

    #[test]
    fn test_ffi_random_keypair() {
        let keypair = ffi_random_keypair();
        let private_key = Fr::from(keypair.private_key);
        let public_key = G1Projective::from(keypair.public_key);
        assert!(public_key.into_affine().is_on_curve());
        // Check the derived key matches the one generated from original key
        assert_eq!(derive_public_key(private_key), public_key);
    }

    #[test]
    fn test_ffi_generate_stealth_commitment() {
        let spending_key = ffi_random_keypair();
        let viewing_key = ffi_random_keypair();

        // generate ephemeral keypair
        let ephemeral_key = ffi_random_keypair();

        let stealth_commitment_payload = ffi_generate_stealth_commitment(
            viewing_key.public_key,
            spending_key.public_key,
            ephemeral_key.private_key,
        );

        let stealth_private_key =
            ffi_generate_stealth_private_key(ephemeral_key.public_key, viewing_key.private_key, spending_key.private_key, stealth_commitment_payload.view_tag);

        if stealth_private_key.is_zero() {
            panic!("View tags did not match");
        }

        let derived_commitment = ffi_derive_public_key(stealth_private_key);
        assert_eq!(derived_commitment, stealth_commitment_payload.stealth_commitment);
    }
}