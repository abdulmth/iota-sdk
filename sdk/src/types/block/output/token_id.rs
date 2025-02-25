// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::output::FoundryId;

impl_id!(pub TokenId, 38, "TODO.");

#[cfg(feature = "serde")]
string_serde_impl!(TokenId);

impl From<FoundryId> for TokenId {
    fn from(foundry_id: FoundryId) -> Self {
        Self::new(*foundry_id)
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::String;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Describes a token id.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct TokenIdDto(pub String);

    impl From<&TokenId> for TokenIdDto {
        fn from(value: &TokenId) -> Self {
            Self(prefix_hex::encode(**value))
        }
    }

    impl TryFrom<&TokenIdDto> for TokenId {
        type Error = Error;

        fn try_from(value: &TokenIdDto) -> Result<Self, Self::Error> {
            Ok(Self::new(
                prefix_hex::decode(&value.0).map_err(|_e| Error::InvalidField("tokenId"))?,
            ))
        }
    }
}
