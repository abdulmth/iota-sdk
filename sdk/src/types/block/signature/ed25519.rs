// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::{fmt, ops::Deref};

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    signatures::ed25519::{PublicKey, Signature, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH},
};

use crate::types::block::{address::Ed25519Address, Error};

/// An Ed25519 signature.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ed25519Signature {
    public_key: [u8; Self::PUBLIC_KEY_LENGTH],
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    signature: [u8; Self::SIGNATURE_LENGTH],
}

impl Ed25519Signature {
    /// The signature kind of an [`Ed25519Signature`].
    pub const KIND: u8 = 0;
    /// Length of an ED25519 public key.
    pub const PUBLIC_KEY_LENGTH: usize = PUBLIC_KEY_LENGTH;
    /// Length of an ED25519 signature.
    pub const SIGNATURE_LENGTH: usize = SIGNATURE_LENGTH;

    /// Creates a new [`Ed25519Signature`].
    pub fn new(public_key: [u8; Self::PUBLIC_KEY_LENGTH], signature: [u8; Self::SIGNATURE_LENGTH]) -> Self {
        Self { public_key, signature }
    }

    /// Returns the public key of an [`Ed25519Signature`].
    pub fn public_key(&self) -> &[u8; Self::PUBLIC_KEY_LENGTH] {
        &self.public_key
    }

    /// Return the actual signature of an [`Ed25519Signature`].
    pub fn signature(&self) -> &[u8; Self::SIGNATURE_LENGTH] {
        &self.signature
    }

    /// Verifies the [`Ed25519Signature`] for a message against an [`Ed25519Address`].
    pub fn is_valid(&self, message: &[u8], address: &Ed25519Address) -> Result<(), Error> {
        let signature_address: [u8; PUBLIC_KEY_LENGTH] = Blake2b256::digest(self.public_key).into();

        if address.deref() != &signature_address {
            return Err(Error::SignaturePublicKeyMismatch {
                expected: prefix_hex::encode(address.as_ref()),
                actual: prefix_hex::encode(signature_address),
            });
        }

        if !PublicKey::try_from_bytes(self.public_key)?.verify(&Signature::from_bytes(self.signature), message) {
            return Err(Error::InvalidSignature);
        }

        Ok(())
    }
}

impl fmt::Debug for Ed25519Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[repr(transparent)]
        struct UnquotedStr<'a>(&'a str);

        impl<'a> fmt::Debug for UnquotedStr<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        f.debug_struct("Ed25519Signature")
            .field("public_key", &UnquotedStr(&prefix_hex::encode(self.public_key)))
            .field("signature", &UnquotedStr(&prefix_hex::encode(self.signature)))
            .finish()
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::String;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Defines an Ed25519 signature.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519SignatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub public_key: String,
        pub signature: String,
    }

    impl From<&Ed25519Signature> for Ed25519SignatureDto {
        fn from(value: &Ed25519Signature) -> Self {
            Self {
                kind: Ed25519Signature::KIND,
                public_key: prefix_hex::encode(value.public_key),
                signature: prefix_hex::encode(value.signature),
            }
        }
    }

    impl TryFrom<&Ed25519SignatureDto> for Ed25519Signature {
        type Error = Error;

        fn try_from(value: &Ed25519SignatureDto) -> Result<Self, Self::Error> {
            Ok(Self::new(
                prefix_hex::decode(&value.public_key).map_err(|_| Error::InvalidField("publicKey"))?,
                prefix_hex::decode(&value.signature).map_err(|_| Error::InvalidField("signature"))?,
            ))
        }
    }
}
