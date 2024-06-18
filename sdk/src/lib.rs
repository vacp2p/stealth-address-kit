#![cfg_attr(docsrs, feature(doc_cfg))]

mod macros;
mod stealth_addresses;

#[cfg(feature = "baby_jub_jub")]
#[cfg_attr(docsrs, doc(cfg(feature = "baby_jub_jub")))]
mod baby_jub_jub_impl;
#[cfg(feature = "bls12_377")]
#[cfg_attr(docsrs, doc(cfg(feature = "bls12_377")))]
mod bls12_377_impl;
#[cfg(feature = "bls12_381")]
#[cfg_attr(docsrs, doc(cfg(feature = "bls12_381")))]
mod bls12_381_impl;
#[cfg(feature = "bn254")]
#[cfg_attr(docsrs, doc(cfg(feature = "bn254")))]
mod bn254_impl;
#[cfg(feature = "bw6_761")]
#[cfg_attr(docsrs, doc(cfg(feature = "bw6_761")))]
mod bw6_761_impl;
#[cfg(feature = "pallas")]
#[cfg_attr(docsrs, doc(cfg(feature = "pallas")))]
mod pallas_impl;
#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
mod secp256k1_impl;
#[cfg(feature = "secp256r1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256r1")))]
mod secp256r1_impl;
#[cfg(feature = "vesta")]
#[cfg_attr(docsrs, doc(cfg(feature = "vesta")))]
mod vesta_impl;

#[cfg(feature = "ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "ffi")))]
mod ffi;

#[cfg(feature = "baby_jub_jub")]
#[cfg_attr(docsrs, doc(cfg(feature = "baby_jub_jub")))]
pub use baby_jub_jub_impl::BabyJubJub;
#[cfg(feature = "pallas")]
#[cfg_attr(docsrs, doc(cfg(feature = "pallas")))]
pub use pallas_impl::Pallas;
#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub use secp256k1_impl::Secp256k1;
#[cfg(feature = "secp256r1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256r1")))]
pub use secp256r1_impl::Secp256r1;
pub use stealth_addresses::StealthAddressOnCurve;
#[cfg(feature = "vesta")]
#[cfg_attr(docsrs, doc(cfg(feature = "vesta")))]
pub use vesta_impl::Vesta;
