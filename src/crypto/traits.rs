//! Trait abstractions of cryptographic operations.
//!
//! Does not contain hashing! Hashes are fixed by the rpm
//! "spec" to sha1, md5 (yes, that is correct), sha2_256.

#[allow(unused)]
use crate::errors::*;
use std::fmt::Debug;

pub mod algorithm {

    pub trait Algorithm: super::Debug {}
    /// currently only RSA is required
    ///
    /// Farsight for future algorithm extensions of rpm
    /// without breaking the API
    #[derive(Debug, Clone, Copy)]
    #[allow(non_camel_case_types)]

    pub struct RSA;

    impl Algorithm for RSA {}
}

/// Signing trait to be implement for RPM signing.
pub trait Signing<A>: Debug
where
    A: algorithm::Algorithm,
    Self::Signature: AsRef<[u8]>,
{
    type Signature;
    fn sign(&self, data: &[u8]) -> Result<Self::Signature, RPMError>;
}

impl<A,T,S> Signing<A> for &T
where
    T: Signing<A,Signature=S>,
    A: algorithm::Algorithm,
    S: AsRef<[u8]>,
{
    type Signature = S;
    fn sign(&self, data: &[u8]) -> Result<Self::Signature, RPMError> {
        T::sign(self, data)
    }
}

/// Verification trait to be implement for RPM signature verification.
pub trait Verifying<A>: Debug
where
    A: algorithm::Algorithm,
    Self::Signature: AsRef<[u8]>,
{
    type Signature;
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), RPMError>;
}



impl<A,T,S> Verifying<A> for &T
where
    T: Verifying<A,Signature=S>,
    A: algorithm::Algorithm,
    S: AsRef<[u8]>,
{
    type Signature = S;
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), RPMError> {
        T::verify(self, data, signature)
    }
}


pub mod key {

    /// Marker trait for key types.
    pub trait KeyType: super::Debug + Copy {}

    /// A secret key that should not be shared with any other party
    /// under any circumstance.
    #[derive(Debug, Clone, Copy)]
    pub struct Secret;

    /// A key publishable to the public.
    #[derive(Debug, Clone, Copy)]
    pub struct Public;

    impl KeyType for Secret {}
    impl KeyType for Public {}
}

/// Implement unreachable signer for empty tuple `()`
impl<A> Signing<A> for std::marker::PhantomData<A>
where
    A: algorithm::Algorithm,
{
    type Signature = Vec<u8>;
    fn sign(&self, _data: &[u8]) -> Result<Self::Signature, RPMError> {
        unreachable!("if you want to verify, you need to implement `sign` of the `Signing` trait")
    }
}

/// Implement unreachable verifier for the empty tuple`()`
impl<A> Verifying<A> for std::marker::PhantomData<A>
where
    A: algorithm::Algorithm,
{
    type Signature = Vec<u8>;
    fn verify(&self, _data: &[u8], _x: &[u8]) -> Result<(), RPMError> {
        unreachable!(
            "if you want to verify, you need to implement `verify` of the `Verifying` trait"
        )
    }
}

#[cfg(test)]
pub(crate) mod test {
    /// Load a pair of sample keys.
    pub(crate) fn load_asc_keys() -> (Vec<u8>, Vec<u8>) {
        let signing_key = include_bytes!("../../test_assets/id_rsa.asc");
        let verification_key = include_bytes!("../../test_assets/id_rsa.pub.asc");
        (signing_key.to_vec(), verification_key.to_vec())
    }
}