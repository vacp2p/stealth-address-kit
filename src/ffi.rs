use crate::stealth_commitments::{
    derive_public_key, generate_random_fr, generate_stealth_commitment,
    generate_stealth_private_key, random_keypair,
};
use ark_bn254::{Fr, G1Projective};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use num_traits::Zero;
use std::ops::Add;
use crate::ffi::CErrorCode::{
    NoError, SerializationErrorInvalidData, SerializationErrorIoError,
    SerializationErrorNotEnoughSpace, SerializationErrorUnexpectedFlags,
};
// we import this to prevent using multiple static libs
#[cfg(feature = "include_rln_ffi")]
#[allow(unused_imports)]
use rln::ffi::*;
#[repr(C)]
#[derive(Debug)]
pub struct CFr([u8; 32]);

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum CErrorCode {
    NoError = 0,
    SerializationErrorNotEnoughSpace = 1,
    SerializationErrorInvalidData = 2,
    SerializationErrorUnexpectedFlags = 3,
    SerializationErrorIoError = 4,
    InvalidKeys = 5,
}

impl From<SerializationError> for CErrorCode {
    fn from(value: SerializationError) -> Self {
        match value {
            SerializationError::NotEnoughSpace => SerializationErrorNotEnoughSpace,
            SerializationError::InvalidData => SerializationErrorInvalidData,
            SerializationError::UnexpectedFlags => SerializationErrorUnexpectedFlags,
            SerializationError::IoError(_) => SerializationErrorIoError,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CReturn<T> {
    value: T,
    err_code: CErrorCode,
}

impl Add for CFr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Fr::try_from(self).unwrap();
        let rhs = Fr::try_from(rhs).unwrap();
        CFr::try_from(lhs.add(rhs)).unwrap()
    }
}

impl Zero for CFr {
    fn zero() -> Self {
        CFr::try_from(Fr::try_from(0).unwrap()).unwrap()
    }

    fn is_zero(&self) -> bool {
        Fr::is_zero(&Fr::from(self))
    }
}

impl TryFrom<Fr> for CFr {
    type Error = SerializationError;

    fn try_from(value: Fr) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        value.serialize_compressed(&mut buf)?;
        let mut res = [0u8; 32];
        res.copy_from_slice(&buf);
        Ok(CFr(res))
    }
}

impl TryFrom<CFr> for Fr {
    type Error = SerializationError;

    fn try_from(value: CFr) -> Result<Self, Self::Error> {
        Fr::deserialize_compressed(value.0.as_slice())
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

impl Add for CG1Projective {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = G1Projective::try_from(self).unwrap();
        let rhs = G1Projective::try_from(rhs).unwrap();
        CG1Projective::try_from(lhs.add(rhs)).unwrap()
    }
}

impl Zero for CG1Projective {
    fn zero() -> Self {
        CG1Projective::try_from(G1Projective::zero()).unwrap()
    }

    fn is_zero(&self) -> bool {
        G1Projective::is_zero(&G1Projective::from(self))
    }
}

impl TryFrom<G1Projective> for CG1Projective {
    type Error = SerializationError;

    fn try_from(value: G1Projective) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        value.serialize_compressed(&mut buf)?;
        let mut result = [0u8; 32];
        result.copy_from_slice(&buf);
        Ok(CG1Projective(result))
    }
}
impl TryFrom<CG1Projective> for G1Projective {
    type Error = SerializationError;

    fn try_from(value: CG1Projective) -> Result<Self, Self::Error> {
        G1Projective::deserialize_compressed(value.0.as_slice())
    }
}

impl From<&CG1Projective> for G1Projective {
    fn from(value: &CG1Projective) -> Self {
        G1Projective::deserialize_compressed(value.0.as_slice()).unwrap()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CKeyPair {
    private_key: CFr,
    public_key: CG1Projective,
}

impl CKeyPair {
    pub fn zero() -> Self {
        CKeyPair {
            private_key: CFr::zero(),
            public_key: CG1Projective::zero(),
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CStealthCommitment {
    stealth_commitment: CG1Projective,
    view_tag: u64,
}

impl CStealthCommitment {
    pub fn zero() -> Self {
        CStealthCommitment {
            stealth_commitment: CG1Projective::zero(),
            view_tag: 0,
        }
    }
}

impl TryFrom<(G1Projective, u64)> for CStealthCommitment {
    type Error = SerializationError;

    fn try_from(value: (G1Projective, u64)) -> Result<Self, Self::Error> {
        Ok(CStealthCommitment {
            stealth_commitment: CG1Projective::try_from(value.0)?,
            view_tag: value.1,
        })
    }
}

impl TryInto<(G1Projective, u64)> for CStealthCommitment {
    type Error = SerializationError;
    fn try_into(self) -> Result<(G1Projective, u64), Self::Error> {
        Ok((self.stealth_commitment.try_into()?, self.view_tag))
    }
}

#[no_mangle]
pub extern "C" fn ffi_generate_random_fr() -> *mut CReturn<CFr> {
    let res = match CFr::try_from(generate_random_fr()) {
        Ok(v) => CReturn {
            value: v,
            err_code: NoError,
        },
        Err(err) => CReturn {
            value: CFr::zero(),
            err_code: err.into(),
        },
    };
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn drop_ffi_generate_random_fr(ptr: *mut CReturn<CFr>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn ffi_derive_public_key(private_key: *mut CFr) -> *mut CReturn<CG1Projective> {
    let private_key = unsafe {
        if private_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CG1Projective::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*private_key
    };
    let private_key: Fr = match private_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CG1Projective::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };

    let res = match CG1Projective::try_from(derive_public_key(private_key)) {
        Ok(v) => CReturn {
            value: v,
            err_code: NoError,
        },
        Err(err) => CReturn {
            value: CG1Projective::zero(),
            err_code: err.into(),
        },
    };
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn drop_ffi_derive_public_key(ptr: *mut CReturn<CG1Projective>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn ffi_random_keypair() -> *mut CReturn<CKeyPair> {
    let (private_key, public_key) = random_keypair();
    let private_key = match CFr::try_from(private_key) {
        Ok(v) => v,
        Err(err) => {
            return Box::into_raw(Box::new(CReturn {
                value: CKeyPair::zero(),
                err_code: err.into(),
            }))
        }
    };
    let public_key = match CG1Projective::try_from(public_key) {
        Ok(v) => v,
        Err(err) => {
            return Box::into_raw(Box::new(CReturn {
                value: CKeyPair::zero(),
                err_code: err.into(),
            }))
        }
    };
    let res = CReturn {
        value: CKeyPair {
            private_key,
            public_key,
        },
        err_code: NoError,
    };
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn drop_ffi_random_keypair(ptr: *mut CReturn<CKeyPair>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn ffi_generate_stealth_commitment(
    viewing_public_key: *mut CG1Projective,
    spending_public_key: *mut CG1Projective,
    ephemeral_private_key: *mut CFr,
) -> *mut CReturn<CStealthCommitment> {
    let viewing_public_key = unsafe {
        if viewing_public_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*viewing_public_key
    };
    let spending_public_key = unsafe {
        if spending_public_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*spending_public_key
    };
    let ephemeral_private_key = unsafe {
        if ephemeral_private_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*ephemeral_private_key
    };

    let viewing_public_key: G1Projective = match viewing_public_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let spending_public_key: G1Projective = match spending_public_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let ephemeral_private_key: Fr = match ephemeral_private_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let res = match CStealthCommitment::try_from(generate_stealth_commitment(
        viewing_public_key,
        spending_public_key,
        ephemeral_private_key,
    )) {
        Ok(v) => CReturn {
            value: v,
            err_code: NoError,
        },
        Err(err) => {
            return Box::into_raw(Box::new(CReturn {
                value: CStealthCommitment::zero(),
                err_code: err.into(),
            }))
        }
    };
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn drop_ffi_generate_stealth_commitment(ptr: *mut CReturn<CStealthCommitment>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn ffi_generate_stealth_private_key(
    ephemeral_public_key: *mut CG1Projective,
    spending_key: *mut CFr,
    viewing_key: *mut CFr,
    view_tag: *mut u64,
) -> *mut CReturn<CFr> {
    let ephemeral_public_key = unsafe {
        if ephemeral_public_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*ephemeral_public_key
    };
    let spending_key = unsafe {
        if spending_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*spending_key
    };
    let viewing_key = unsafe {
        if viewing_key.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*viewing_key
    };
    let view_tag = unsafe {
        if view_tag.is_null() {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }));
        }
        &*view_tag
    };

    let ephemeral_public_key: G1Projective = match ephemeral_public_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let spending_key: Fr = match spending_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let viewing_key: Fr = match viewing_key.try_into() {
        Ok(v) => v,
        Err(_) => {
            return Box::into_raw(Box::new(CReturn {
                value: CFr::zero(),
                err_code: CErrorCode::InvalidKeys,
            }))
        }
    };
    let stealth_private_key_opt =
        generate_stealth_private_key(ephemeral_public_key, spending_key, viewing_key, *view_tag);
    if stealth_private_key_opt.is_none() {
        return Box::into_raw(Box::new(CReturn {
            value: CFr::zero(),
            err_code: CErrorCode::InvalidKeys,
        }));
    }
    let res = match CFr::try_from(stealth_private_key_opt.unwrap()) {
        Ok(v) => CReturn {
            value: v,
            err_code: NoError,
        },
        Err(err) => CReturn {
            value: CFr::zero(),
            err_code: err.into(),
        },
    };
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn drop_ffi_generate_stealth_private_key(ptr: *mut CReturn<CFr>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::stealth_commitments::derive_public_key;
    use ark_ec::CurveGroup;

    #[test]
    fn test_ffi_generate_random_fr() {
        let _ = ffi_generate_random_fr();
    }

    #[test]
    fn test_ffi_random_keypair() {
        // Generate a random keypair
        let keypair_raw = ffi_random_keypair();
        let keypair = unsafe { &*keypair_raw };

        // Extract private and public keys
        let private_key = Fr::try_from(&keypair.value.private_key).unwrap();
        let public_key = G1Projective::try_from(&keypair.value.public_key).unwrap();

        // Drop the keypair to avoid memory leaks
        drop_ffi_random_keypair(keypair_raw);

        // Assert that the public key is on the curve
        assert!(public_key.into_affine().is_on_curve());

        // Check if the derived key matches the one generated from the original key
        assert_eq!(derive_public_key(private_key), public_key);
    }

    #[test]
    fn test_ffi_generate_stealth_commitment() {
        // Generate random keypairs
        let spending_key_raw = ffi_random_keypair();
        let spending_key = unsafe { &mut *spending_key_raw };
        let viewing_key_raw = ffi_random_keypair();
        let viewing_key = unsafe { &mut *viewing_key_raw };
        let ephemeral_key_raw = ffi_random_keypair();
        let ephemeral_key = unsafe { &mut *ephemeral_key_raw };

        // Extract pointers
        let viewing_pub_key_ptr = &mut viewing_key.value.public_key;
        let viewing_priv_key_ptr = &mut viewing_key.value.private_key;
        let spending_pub_key_ptr = &mut spending_key.value.public_key;
        let spending_priv_key_ptr = &mut spending_key.value.private_key;
        let ephemeral_pub_key_ptr = &mut ephemeral_key.value.public_key;
        let ephemeral_priv_key_ptr = &mut ephemeral_key.value.private_key;

        // Generate stealth commitment payload
        let stealth_commitment_payload_raw = ffi_generate_stealth_commitment(
            viewing_pub_key_ptr,
            spending_pub_key_ptr,
            ephemeral_priv_key_ptr,
        );
        let stealth_commitment_payload = unsafe { &mut *stealth_commitment_payload_raw };
        let view_tag_ptr = &mut stealth_commitment_payload.value.view_tag;

        // Generate stealth private key
        let stealth_private_key_raw = ffi_generate_stealth_private_key(
            ephemeral_pub_key_ptr,
            viewing_priv_key_ptr,
            spending_priv_key_ptr,
            view_tag_ptr,
        );

        drop_ffi_random_keypair(ephemeral_key_raw);
        drop_ffi_random_keypair(viewing_key_raw);
        drop_ffi_random_keypair(spending_key_raw);

        let stealth_private_key = unsafe { &mut *stealth_private_key_raw };
        // Check for errors
        if stealth_private_key.err_code != NoError {
            panic!("View tags did not match");
        }

        // Derive commitment
        let derived_commitment_raw = ffi_derive_public_key(&mut stealth_private_key.value);
        drop_ffi_generate_stealth_private_key(stealth_private_key_raw);

        let derived_commitment = unsafe { &*derived_commitment_raw };

        assert_eq!(
            derived_commitment.value,
            stealth_commitment_payload.value.stealth_commitment
        );
        // Drop all allocated memory to avoid memory leaks
        drop_ffi_generate_stealth_commitment(stealth_commitment_payload_raw);
        drop_ffi_derive_public_key(derived_commitment_raw);
    }
}
