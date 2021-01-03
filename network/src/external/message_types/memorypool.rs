// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    errors::message::MessageError,
    external::message::{Message, MessageName},
};

#[cfg_attr(nightly, doc(include = "../../../documentation/network_messages/memory_pool.md"))]
#[derive(Debug, PartialEq, Clone)]
pub struct MemoryPool {
    /// Vector of memory pool transactions
    pub transactions: Vec<Vec<u8>>,
}

impl MemoryPool {
    pub fn new(transactions: Vec<Vec<u8>>) -> Self {
        Self { transactions }
    }
}

impl Message for MemoryPool {
    fn name() -> MessageName {
        MessageName::from("memorypool")
    }

    fn deserialize(vec: Vec<u8>) -> Result<Self, MessageError> {
        Ok(Self {
            transactions: bincode::deserialize(&vec)?,
        })
    }

    fn serialize(&self) -> Result<Vec<u8>, MessageError> {
        Ok(bincode::serialize(&self.transactions)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkos_testing::consensus::TRANSACTION_1;

    #[test]
    fn test_memory_pool() {
        let message = MemoryPool::new(vec![TRANSACTION_1.to_vec()]);

        let serialized = message.serialize().unwrap();
        let deserialized = MemoryPool::deserialize(serialized).unwrap();

        assert_eq!(message, deserialized);
    }
}