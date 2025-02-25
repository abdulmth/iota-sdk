// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Identifies the validated issuer of the UTXO state machine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IssuerFeature(Address);

impl IssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`IssuerFeature`].
    pub const KIND: u8 = 1;

    /// Creates a new [`IssuerFeature`].
    #[inline(always)]
    pub fn new(address: Address) -> Self {
        Self(address)
    }

    /// Returns the issuer [`Address`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use crate::types::block::address::dto::AddressDto;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct IssuerFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }
}
