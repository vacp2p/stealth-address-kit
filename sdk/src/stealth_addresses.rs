use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_ff::{Fp, FpConfig, PrimeField};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::rngs::OsRng;
use ark_std::UniformRand;
use std::fmt::Display;
use std::ops::Add;
use tiny_keccak::{Hasher, Keccak};

/// A trait for types that have a view tag.
pub trait HasViewTag {
    /// Returns the view tag.
    fn get_view_tag(&self) -> u64;
}

impl<P: FpConfig<N>, const N: usize> HasViewTag for Fp<P, N>
where
    Fp<P, N>: PrimeField,
{
    fn get_view_tag(&self) -> u64 {
        self.0 .0[0]
    }
}

/// A trait for converting projective points to bytes.
pub trait ToBytesFromProjective {
    /// Converts the projective point to a byte vector.
    fn to_bytes(&self) -> Vec<u8>;
}

impl<G: CurveGroup> ToBytesFromProjective for G
where
    G::Affine: CanonicalSerialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        let affine = self.into_affine();
        let mut bytes = Vec::with_capacity(affine.compressed_size());
        affine.serialize_compressed(&mut bytes).unwrap();
        bytes
    }
}

// we want to route through CurveGroup -> Config -> ScalarField
type FrOf<P> =
    <<<P as StealthAddressOnCurve>::Projective as CurveGroup>::Affine as AffineRepr>::ScalarField;

/// A trait for implementing stealth addresses on elliptic curves.
pub trait StealthAddressOnCurve {
    /// The projective representation of the elliptic curve point.
    type Projective: Display
        + Add<Output = Self::Projective>
        + From<<Self::Projective as CurveGroup>::Affine>
        + CurveGroup;

    /// Derives a public key from a given private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - A reference to the private key.
    ///
    /// # Returns
    ///
    /// The derived public key.
    #[inline]
    fn derive_public_key(private_key: &FrOf<Self>) -> Self::Projective {
        Self::Projective::generator() * *private_key
    }

    /// Generates a random keypair.
    ///
    /// # Returns
    ///
    /// A tuple containing the private key and the derived public key.
    #[inline]
    fn random_keypair() -> (FrOf<Self>, Self::Projective) {
        let private_key = Self::generate_random_fr();
        let public_key = Self::derive_public_key(&private_key);
        (private_key, public_key)
    }

    /// Generates a random scalar field element.
    ///
    /// # Returns
    ///
    /// A random scalar field element.
    #[inline]
    fn generate_random_fr() -> FrOf<Self> {
        FrOf::<Self>::rand(&mut OsRng)
    }

    /// Hashes an input byte slice to a scalar field element.
    ///
    /// # Arguments
    ///
    /// * `input` - A byte slice to be hashed.
    ///
    /// # Returns
    ///
    /// A scalar field element derived from the hash of the input.
    #[inline]
    fn hash_to_fr(input: &[u8]) -> FrOf<Self>
    where
        FrOf<Self>: HasViewTag,
    {
        let mut hash = [0; 32];
        let mut hasher = Keccak::v256();
        hasher.update(input);
        hasher.finalize(&mut hash);

        // We export the hash as a field element
        FrOf::<Self>::from_le_bytes_mod_order(hash.as_slice())
    }

    /// Computes a shared elliptic curve point given a private key and a public key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key.
    /// * `public_key` - The public key.
    ///
    /// # Returns
    ///
    /// The computed shared elliptic curve point.
    #[inline]
    fn compute_shared_point(
        private_key: FrOf<Self>,
        public_key: Self::Projective,
    ) -> Self::Projective {
        public_key * private_key
    }

    /// Generates a stealth address.
    ///
    /// # Arguments
    ///
    /// * `viewing_public_key` - The viewing public key.
    /// * `spending_public_key` - The spending public key.
    /// * `ephemeral_private_key` - The ephemeral private key.
    ///
    /// # Returns
    ///
    /// A tuple containing the stealth address and the view tag.
    #[inline]
    fn generate_stealth_address(
        viewing_public_key: Self::Projective,
        spending_public_key: Self::Projective,
        ephemeral_private_key: FrOf<Self>,
    ) -> (Self::Projective, u64)
    where
        FrOf<Self>: HasViewTag,
    {
        let q = Self::compute_shared_point(ephemeral_private_key, viewing_public_key);
        let q_hashed = Self::hash_to_fr(&q.to_bytes());
        let q_hashed_in_g1 = Self::derive_public_key(&q_hashed);
        let view_tag = q_hashed.get_view_tag();
        (q_hashed_in_g1 + spending_public_key, view_tag)
    }

    /// Generates a stealth private key.
    ///
    /// # Arguments
    ///
    /// * `ephemeral_public_key` - The ephemeral public key.
    /// * `viewing_key` - The viewing key.
    /// * `spending_key` - The spending key.
    /// * `expected_view_tag` - The expected view tag.
    ///
    /// # Returns
    ///
    /// An optional stealth private key.
    #[inline]
    fn generate_stealth_private_key(
        ephemeral_public_key: Self::Projective,
        viewing_key: FrOf<Self>,
        spending_key: FrOf<Self>,
        expected_view_tag: u64,
    ) -> Option<FrOf<Self>>
    where
        FrOf<Self>: HasViewTag,
    {
        let q_receiver = Self::compute_shared_point(viewing_key, ephemeral_public_key);
        let q_receiver_hashed = Self::hash_to_fr(&q_receiver.to_bytes());
        if q_receiver_hashed.get_view_tag() == expected_view_tag {
            Some(spending_key + q_receiver_hashed)
        } else {
            None
        }
    }
}
