#[macro_export]
macro_rules! define_curve_ffi {
    ($curve_name:ident, $Curve:ty, $Fr:ty, $G1Projective:ty, $FR_SIZE: expr, $PROJECTIVE_SIZE:expr) => {
        use paste::paste;
        use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
        use num_traits::Zero;
        use std::ops::Add;

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
                    SerializationError::NotEnoughSpace => CErrorCode::SerializationErrorNotEnoughSpace,
                    SerializationError::InvalidData => CErrorCode::SerializationErrorInvalidData,
                    SerializationError::UnexpectedFlags => CErrorCode::SerializationErrorUnexpectedFlags,
                    SerializationError::IoError(_) => CErrorCode::SerializationErrorIoError,
                }
            }
        }

        #[repr(C)]
        #[derive(Debug)]
        pub struct CReturn<T> {
            value: T,
            err_code: CErrorCode,
        }
        paste! {
            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name _Fr>]([u8; $FR_SIZE]);

            #[repr(C)]
            #[derive(Debug, PartialOrd, PartialEq)]
            pub struct [<$curve_name _G1Projective>]([u8; $PROJECTIVE_SIZE]);

            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name _KeyPair>] {
                private_key: [<$curve_name _Fr>],
                public_key: [<$curve_name _G1Projective>],
            }

            #[repr(C)]
            #[derive(Debug)]
            pub struct [<$curve_name _StealthAddress>] {
                stealth_address: [<$curve_name _G1Projective>],
                view_tag: u64,
            }

            impl Add for [<$curve_name _Fr>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let lhs = <$Fr>::try_from(self).unwrap();
                    let rhs = <$Fr>::try_from(rhs).unwrap();
                    [<$curve_name _Fr>]::try_from(lhs.add(rhs)).unwrap()
                }
            }

            impl Zero for [<$curve_name _Fr>] {
                fn zero() -> Self {
                    [<$curve_name _Fr>]::try_from(<$Fr>::try_from(0).unwrap()).unwrap()
                }

                fn is_zero(&self) -> bool {
                    <$Fr>::is_zero(&<$Fr>::from(self))
                }
            }

            impl TryFrom<$Fr> for [<$curve_name _Fr>] {
                type Error = SerializationError;

                fn try_from(value: $Fr) -> Result<Self, Self::Error> {
                    let mut buf = Vec::new();
                    value.serialize_compressed(&mut buf)?;
                    let mut res = [0u8; $FR_SIZE];
                    res.copy_from_slice(&buf);
                    Ok([<$curve_name _Fr>](res))
                }
            }

            impl TryFrom<[<$curve_name _Fr>]> for $Fr {
                type Error = SerializationError;

                fn try_from(value: [<$curve_name _Fr>]) -> Result<Self, Self::Error> {
                    <$Fr>::deserialize_compressed(value.0.as_slice())
                }
            }

            impl From<&[<$curve_name _Fr>]> for $Fr {
                fn from(value: &[<$curve_name _Fr>]) -> Self {
                    <$Fr>::deserialize_compressed(value.0.as_slice()).unwrap()
                }
            }

            impl Add for [<$curve_name _G1Projective>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let lhs = <$G1Projective>::try_from(self).unwrap();
                    let rhs = <$G1Projective>::try_from(rhs).unwrap();
                    <[<$curve_name _G1Projective>]>::try_from(lhs.add(rhs)).unwrap()
                }
            }

            impl Zero for [<$curve_name _G1Projective>] {
                fn zero() -> Self {
                    <[<$curve_name _G1Projective>]>::try_from(<$G1Projective>::zero()).unwrap()
                }

                fn is_zero(&self) -> bool {
                    <$G1Projective>::is_zero(&<$G1Projective>::from(self))
                }
            }

            impl TryFrom<$G1Projective> for [<$curve_name _G1Projective>] {
                type Error = SerializationError;

                fn try_from(value: $G1Projective) -> Result<Self, Self::Error> {
                    let mut buf = Vec::new();
                    value.serialize_compressed(&mut buf)?;
                    let mut result = [0u8; $PROJECTIVE_SIZE];
                    result.copy_from_slice(&buf);
                    Ok([<$curve_name _G1Projective>](result))
                }
            }

            impl TryFrom<[<$curve_name _G1Projective>]> for $G1Projective {
                type Error = SerializationError;

                fn try_from(value: [<$curve_name _G1Projective>]) -> Result<Self, Self::Error> {
                    <$G1Projective>::deserialize_compressed(value.0.as_slice())
                }
            }

            impl From<&[<$curve_name _G1Projective>]> for $G1Projective {
                fn from(value: &[<$curve_name _G1Projective>]) -> Self {
                    <$G1Projective>::deserialize_compressed(value.0.as_slice()).unwrap()
                }
            }

            impl [<$curve_name _KeyPair>] {
                pub fn zero() -> Self {
                    [<$curve_name _KeyPair>] {
                        private_key: [<$curve_name _Fr>]::zero(),
                        public_key: [<$curve_name _G1Projective>]::zero(),
                    }
                }
            }

            impl [<$curve_name _StealthAddress>] {
                pub fn zero() -> Self {
                    [<$curve_name _StealthAddress>] {
                        stealth_address: [<$curve_name _G1Projective>]::zero(),
                        view_tag: 0,
                    }
                }
            }

            impl TryFrom<($G1Projective, u64)> for [<$curve_name _StealthAddress>] {
                type Error = SerializationError;

                fn try_from(value: ($G1Projective, u64)) -> Result<Self, Self::Error> {
                    Ok([<$curve_name _StealthAddress>] {
                        stealth_address: <[<$curve_name _G1Projective>]>::try_from(value.0)?,
                        view_tag: value.1,
                    })
                }
            }

            impl TryInto<($G1Projective, u64)> for [<$curve_name _StealthAddress>] {
                type Error = SerializationError;
                fn try_into(self) -> Result<($G1Projective, u64), Self::Error> {
                    Ok((self.stealth_address.try_into()?, self.view_tag))
                }
            }
            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_random_fr>]() -> *mut CReturn<[<$curve_name _Fr>]> {
                let res = match [<$curve_name _Fr>]::try_from(<$Curve>::generate_random_fr()) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => CReturn {
                        value: [<$curve_name _Fr>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_random_fr>](ptr: *mut CReturn<[<$curve_name _Fr>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_derive_public_key>](private_key: *mut [<$curve_name _Fr>]) -> *mut CReturn<[<$curve_name _G1Projective>]> {
                let private_key = unsafe {
                    if private_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _G1Projective>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*private_key
                };
                let private_key: $Fr = match private_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _G1Projective>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };

                let res = match [<$curve_name _G1Projective>]::try_from(<$Curve>::derive_public_key(&private_key)) {
                    Ok(v) => CReturn {
                        value: v,
                        err_code: CErrorCode::NoError,
                    },
                    Err(err) => CReturn {
                        value: [<$curve_name _G1Projective>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_derive_public_key>](ptr: *mut CReturn<[<$curve_name _G1Projective>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_random_keypair>]() -> *mut CReturn<[<$curve_name _KeyPair>]> {
                let (private_key, public_key) = <$Curve>::random_keypair();
                let private_key = match [<$curve_name _Fr>]::try_from(private_key) {
                    Ok(v) => v,
                    Err(err) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _KeyPair>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                let public_key = match [<$curve_name _G1Projective>]::try_from(public_key) {
                    Ok(v) => v,
                    Err(err) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _KeyPair>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                let res = CReturn {
                    value: [<$curve_name _KeyPair>] {
                        private_key,
                        public_key,
                    },
                    err_code: CErrorCode::NoError,
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_random_keypair>](ptr: *mut CReturn<[<$curve_name _KeyPair>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_stealth_address>](
                viewing_public_key: *mut [<$curve_name _G1Projective>],
                spending_public_key: *mut [<$curve_name _G1Projective>],
                ephemeral_private_key: *mut [<$curve_name _Fr>],
            ) -> *mut CReturn<[<$curve_name _StealthAddress>]> {
                let viewing_public_key = unsafe {
                    if viewing_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*viewing_public_key
                };
                let spending_public_key = unsafe {
                    if spending_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*spending_public_key
                };
                let ephemeral_private_key = unsafe {
                    if ephemeral_private_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*ephemeral_private_key
                };

                let viewing_public_key: $G1Projective = match viewing_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let spending_public_key: $G1Projective = match spending_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let ephemeral_private_key: $Fr = match ephemeral_private_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let res = match [<$curve_name _StealthAddress>]::try_from(<$Curve>::generate_stealth_address(
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
                            value: [<$curve_name _StealthAddress>]::zero(),
                            err_code: err.into(),
                        }))
                    }
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_stealth_address>](ptr: *mut CReturn<[<$curve_name _StealthAddress>]>) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }

            #[no_mangle]
            pub extern "C" fn [<$curve_name _ffi_generate_stealth_private_key>](
                ephemeral_public_key: *mut [<$curve_name _G1Projective>],
                spending_key: *mut [<$curve_name _Fr>],
                    viewing_key: *mut [<$curve_name _Fr>],
                view_tag: *mut u64,
            ) -> *mut CReturn<[<$curve_name _Fr>]> {
                let ephemeral_public_key = unsafe {
                    if ephemeral_public_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*ephemeral_public_key
                };
                let spending_key = unsafe {
                    if spending_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*spending_key
                };
                let viewing_key = unsafe {
                    if viewing_key.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*viewing_key
                };
                let view_tag = unsafe {
                    if view_tag.is_null() {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }));
                    }
                    &*view_tag
                };

                let ephemeral_public_key: $G1Projective = match ephemeral_public_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let spending_key: $Fr = match spending_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };
                let viewing_key: $Fr = match viewing_key.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Box::into_raw(Box::new(CReturn {
                            value: [<$curve_name _Fr>]::zero(),
                            err_code: CErrorCode::InvalidKeys,
                        }))
                    }
                };

                let res = match [<$curve_name _Fr>]::try_from(<$Curve>::generate_stealth_private_key(
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
                        value: [<$curve_name _Fr>]::zero(),
                        err_code: err.into(),
                    },
                };
                Box::into_raw(Box::new(res))
            }

            #[no_mangle]
            pub extern "C" fn [<drop_ $curve_name _ffi_generate_stealth_private_key>](ptr: *mut CReturn<[<$curve_name _Fr>]>) {
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
                    let public_key = $G1Projective::try_from(&keypair.value.public_key).unwrap();

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
