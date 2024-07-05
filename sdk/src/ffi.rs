#[macro_export]
macro_rules! define_curve_ffi {
    ($curve_name:ident, $Curve:ty, $Fr:ty, $Projective:ty, $FR_SIZE: expr, $PROJECTIVE_SIZE:expr) => {
        use paste::paste;
        use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
        use num_traits::Zero;
        use std::ops::Add;
        use crate::ffi_prelude::{CReturn, CErrorCode};

        paste! {
            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name Fr>]([u8; $FR_SIZE]);

            #[repr(C)]
            #[derive(Debug, PartialOrd, PartialEq)]
            pub struct [<$curve_name Projective>]([u8; $PROJECTIVE_SIZE]);

            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name KeyPair>] {
                private_key: [<$curve_name Fr>],
                public_key: [<$curve_name Projective>],
            }

            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name StealthAddress>] {
                stealth_address: [<$curve_name Projective>],
                view_tag: u64,
            }

            impl Add for [<$curve_name Fr>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let lhs = <$Fr>::try_from(self).unwrap();
                    let rhs = <$Fr>::try_from(rhs).unwrap();
                    [<$curve_name Fr>]::try_from(lhs.add(rhs)).unwrap()
                }
            }

            impl Zero for [<$curve_name Fr>] {
                fn zero() -> Self {
                    [<$curve_name Fr>]::try_from(<$Fr>::try_from(0).unwrap()).unwrap()
                }

                fn is_zero(&self) -> bool {
                    <$Fr>::is_zero(&<$Fr>::from(self))
                }
            }

            impl TryFrom<$Fr> for [<$curve_name Fr>] {
                type Error = SerializationError;

                fn try_from(value: $Fr) -> Result<Self, Self::Error> {
                    let mut buf = Vec::new();
                    value.serialize_compressed(&mut buf)?;
                    let mut res = [0u8; $FR_SIZE];
                    res.copy_from_slice(&buf);
                    Ok([<$curve_name Fr>](res))
                }
            }

            impl TryFrom<[<$curve_name Fr>]> for $Fr {
                type Error = SerializationError;

                fn try_from(value: [<$curve_name Fr>]) -> Result<Self, Self::Error> {
                    <$Fr>::deserialize_compressed(value.0.as_slice())
                }
            }

            impl From<&[<$curve_name Fr>]> for $Fr {
                fn from(value: &[<$curve_name Fr>]) -> Self {
                    <$Fr>::deserialize_compressed(value.0.as_slice()).unwrap()
                }
            }

            impl Add for [<$curve_name Projective>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let lhs = <$Projective>::try_from(self).unwrap();
                    let rhs = <$Projective>::try_from(rhs).unwrap();
                    <[<$curve_name Projective>]>::try_from(lhs.add(rhs)).unwrap()
                }
            }

            impl Zero for [<$curve_name Projective>] {
                fn zero() -> Self {
                    <[<$curve_name Projective>]>::try_from(<$Projective>::zero()).unwrap()
                }

                fn is_zero(&self) -> bool {
                    <$Projective>::is_zero(&<$Projective>::from(self))
                }
            }

            impl TryFrom<$Projective> for [<$curve_name Projective>] {
                type Error = SerializationError;

                fn try_from(value: $Projective) -> Result<Self, Self::Error> {
                    let mut buf = Vec::new();
                    value.serialize_compressed(&mut buf)?;
                    let mut result = [0u8; $PROJECTIVE_SIZE];
                    result.copy_from_slice(&buf);
                    Ok([<$curve_name Projective>](result))
                }
            }

            impl TryFrom<[<$curve_name Projective>]> for $Projective {
                type Error = SerializationError;

                fn try_from(value: [<$curve_name Projective>]) -> Result<Self, Self::Error> {
                    <$Projective>::deserialize_compressed(value.0.as_slice())
                }
            }

            impl From<&[<$curve_name Projective>]> for $Projective {
                fn from(value: &[<$curve_name Projective>]) -> Self {
                    <$Projective>::deserialize_compressed(value.0.as_slice()).unwrap()
                }
            }

            impl [<$curve_name KeyPair>] {
                pub fn zero() -> Self {
                    [<$curve_name KeyPair>] {
                        private_key: [<$curve_name Fr>]::zero(),
                        public_key: [<$curve_name Projective>]::zero(),
                    }
                }
            }

            impl [<$curve_name StealthAddress>] {
                pub fn zero() -> Self {
                    [<$curve_name StealthAddress>] {
                        stealth_address: [<$curve_name Projective>]::zero(),
                        view_tag: 0,
                    }
                }
            }

            impl TryFrom<($Projective, u64)> for [<$curve_name StealthAddress>] {
                type Error = SerializationError;

                fn try_from(value: ($Projective, u64)) -> Result<Self, Self::Error> {
                    Ok([<$curve_name StealthAddress>] {
                        stealth_address: <[<$curve_name Projective>]>::try_from(value.0)?,
                        view_tag: value.1,
                    })
                }
            }

            impl TryInto<($Projective, u64)> for [<$curve_name StealthAddress>] {
                type Error = SerializationError;
                fn try_into(self) -> Result<($Projective, u64), Self::Error> {
                    Ok((self.stealth_address.try_into()?, self.view_tag))
                }
            }
            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_random_fr>]() -> *mut CReturn<[<$curve_name Fr>]> {
                let res = match [<$curve_name Fr>]::try_from(<$Curve>::generate_random_fr()) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => CReturn {
                        value: [<$curve_name Fr>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_random_fr>](ptr: *mut CReturn<[<$curve_name Fr>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_derive_public_key>](private_key: *mut [<$curve_name Fr>]) -> *mut CReturn<[<$curve_name Projective>]> {
                let private_key = unsafe {
                    if private_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Projective>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*private_key
                };
                let private_key: $Fr = match private_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Projective>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };

                let res = match [<$curve_name Projective>]::try_from(<$Curve>::derive_public_key(&private_key)) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => CReturn {
                        value: [<$curve_name Projective>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_derive_public_key>](ptr: *mut CReturn<[<$curve_name Projective>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_random_keypair>]() -> *mut CReturn<[<$curve_name KeyPair>]> {
                let (private_key, public_key) = <$Curve>::random_keypair();
                let private_key = match [<$curve_name Fr>]::try_from(private_key) {
                    Ok(v) => v,
                    Err(err) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name KeyPair>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                let public_key = match [<$curve_name Projective>]::try_from(public_key) {
                    Ok(v) => v,
                    Err(err) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name KeyPair>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                let res = CReturn {
                    value: [<$curve_name KeyPair>] {
                        private_key,
                        public_key,
                    },
                    err_code: CErrorCode::NoError,
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_random_keypair>](ptr: *mut CReturn<[<$curve_name KeyPair>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_stealth_address>](
                viewing_public_key: *mut [<$curve_name Projective>],
                spending_public_key: *mut [<$curve_name Projective>],
                ephemeral_private_key: *mut [<$curve_name Fr>],
            ) -> *mut CReturn<[<$curve_name StealthAddress>]> {
                let viewing_public_key = unsafe {
                    if viewing_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*viewing_public_key
                };
                let spending_public_key = unsafe {
                    if spending_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*spending_public_key
                };
                let ephemeral_private_key = unsafe {
                    if ephemeral_private_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*ephemeral_private_key
                };

                let viewing_public_key: $Projective = match viewing_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let spending_public_key: $Projective = match spending_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let ephemeral_private_key: $Fr = match ephemeral_private_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let res = match [<$curve_name StealthAddress>]::try_from(<$Curve>::generate_stealth_address(
                    viewing_public_key,
                    spending_public_key,
                    ephemeral_private_key,
                )) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name StealthAddress>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_stealth_address>](ptr: *mut CReturn<[<$curve_name StealthAddress>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_stealth_private_key>](
                ephemeral_public_key: *mut [<$curve_name Projective>],
                spending_key: *mut [<$curve_name Fr>],
                    viewing_key: *mut [<$curve_name Fr>],
                view_tag: *mut u64,
            ) -> *mut CReturn<[<$curve_name Fr>]> {
                let ephemeral_public_key = unsafe {
                    if ephemeral_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*ephemeral_public_key
                };
                let spending_key = unsafe {
                    if spending_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*spending_key
                };
                let viewing_key = unsafe {
                    if viewing_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*viewing_key
                };
                let view_tag = unsafe {
                    if view_tag.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*view_tag
                };

                let ephemeral_public_key: $Projective = match ephemeral_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let spending_key: $Fr = match spending_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let viewing_key: $Fr = match viewing_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };

                let res = match [<$curve_name Fr>]::try_from(<$Curve>::generate_stealth_private_key(
                    ephemeral_public_key,
                    spending_key,
                    viewing_key,
                    *view_tag,
                ).unwrap()) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => CReturn {
                        value: [<$curve_name Fr>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_stealth_private_key>](ptr: *mut CReturn<[<$curve_name Fr>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[cfg(test)]
            mod ffi_tests {

                use super::*;
                use ark_ec::CurveGroup;

                #[test]
                fn generate_random_fr_happy_path() {
                    let _ = [<$curve_name _ffi_generate_random_fr>]();
                }

                #[test]
                fn random_keypair_happy_path() {
                    // Generate a random keypair
                    let keypair_raw = [<$curve_name _ffi_random_keypair>]();
                    let keypair = unsafe { &*keypair_raw };

                    // Extract private and public keys
                    let private_key = $Fr::try_from(&keypair.value.private_key).unwrap();
                    let public_key = $Projective::try_from(&keypair.value.public_key).unwrap();

                    // Drop the keypair to avoid memory leaks
                    [<drop_ $curve_name _ffi_random_keypair>](keypair_raw);

                    // Assert that the public key is on the curve
                    assert!(public_key.into_affine().is_on_curve());

                    // Check if the derived key matches the one generated from the original key
                    assert_eq!($Curve::derive_public_key(&private_key), public_key);
                }

                #[test]
                fn generate_stealth_address_happy_path() {
                    // Generate random keypairs
                    let spending_key_raw = [<$curve_name _ffi_random_keypair>]();
                    let spending_key = unsafe { &mut *spending_key_raw };
                    let viewing_key_raw = [<$curve_name _ffi_random_keypair>]();
                    let viewing_key = unsafe { &mut *viewing_key_raw };
                    let ephemeral_key_raw = [<$curve_name _ffi_random_keypair>]();
                    let ephemeral_key = unsafe { &mut *ephemeral_key_raw };

                    // Extract pointers
                    let viewing_pub_key_ptr = &mut viewing_key.value.public_key;
                    let viewing_priv_key_ptr = &mut viewing_key.value.private_key;
                    let spending_pub_key_ptr = &mut spending_key.value.public_key;
                    let spending_priv_key_ptr = &mut spending_key.value.private_key;
                    let ephemeral_pub_key_ptr = &mut ephemeral_key.value.public_key;
                    let ephemeral_priv_key_ptr = &mut ephemeral_key.value.private_key;

                    // Generate stealth address payload
                    let stealth_address_payload_raw = [<$curve_name _ffi_generate_stealth_address>](
                        viewing_pub_key_ptr,
                        spending_pub_key_ptr,
                        ephemeral_priv_key_ptr,
                    );
                    let stealth_address_payload = unsafe { &mut *stealth_address_payload_raw };
                    let view_tag_ptr = &mut stealth_address_payload.value.view_tag;

                    // Generate stealth private key
                    let stealth_private_key_raw = [<$curve_name _ffi_generate_stealth_private_key>](
                        ephemeral_pub_key_ptr,
                        viewing_priv_key_ptr,
                        spending_priv_key_ptr,
                        view_tag_ptr,
                    );

                    [<drop_ $curve_name _ffi_random_keypair>](ephemeral_key_raw);
                    [<drop_ $curve_name _ffi_random_keypair>](viewing_key_raw);
                    [<drop_ $curve_name _ffi_random_keypair>](spending_key_raw);

                    let stealth_private_key = unsafe { &mut *stealth_private_key_raw };
                    // Check for errors
                    if stealth_private_key.err_code != CErrorCode::NoError {
                        panic!("View tags did not match");
                    }

                    // Derive address
                    let derived_address_raw = [<$curve_name _ffi_derive_public_key>](&mut stealth_private_key.value);
                    [<drop_ $curve_name _ffi_generate_stealth_private_key>](stealth_private_key_raw);

                    let derived_address = unsafe { &*derived_address_raw };

                    assert_eq!(
                        derived_address.value,
                        stealth_address_payload.value.stealth_address
                    );
                    // Drop all allocated memory to avoid memory leaks
                     [<drop_ $curve_name _ffi_generate_stealth_address>](stealth_address_payload_raw);
                     [<drop_ $curve_name _ffi_derive_public_key>](derived_address_raw);
                }
            }

        }

    };
}
