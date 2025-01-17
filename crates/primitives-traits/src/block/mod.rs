//! Block abstraction.

pub mod body;
pub mod header;

use alloc::fmt;
use alloy_consensus::Header;
use alloy_rlp::{Decodable, Encodable};

use crate::{
    BlockBody, BlockHeader, FullBlockBody, FullBlockHeader, InMemorySize, MaybeSerde,
    SignedTransaction,
};

/// Helper trait that unifies all behaviour required by block to support full node operations.
pub trait FullBlock:
    Block<Header: FullBlockHeader, Body: FullBlockBody> + alloy_rlp::Encodable + alloy_rlp::Decodable
{
}

impl<T> FullBlock for T where
    T: Block<Header: FullBlockHeader, Body: FullBlockBody>
        + alloy_rlp::Encodable
        + alloy_rlp::Decodable
{
}

/// Helper trait to access [`BlockBody::Transaction`] given a [`Block`].
pub type BlockTx<B> = <<B as Block>::Body as BlockBody>::Transaction;

/// Abstraction of block data type.
// todo: make sealable super-trait, depends on <https://github.com/paradigmxyz/reth/issues/11449>
// todo: make with senders extension trait, so block can be impl by block type already containing
// senders
pub trait Block:
    Send
    + Sync
    + Unpin
    + Clone
    + Default
    + fmt::Debug
    + PartialEq
    + Eq
    + InMemorySize
    + MaybeSerde
    + Encodable
    + Decodable
{
    /// Header part of the block.
    type Header: BlockHeader;

    /// The block's body contains the transactions in the block.
    type Body: BlockBody<OmmerHeader = Self::Header>;

    /// Create new block instance.
    fn new(header: Self::Header, body: Self::Body) -> Self;

    /// Returns reference to block header.
    fn header(&self) -> &Self::Header;

    /// Returns reference to block body.
    fn body(&self) -> &Self::Body;

    /// Splits the block into its header and body.
    fn split(self) -> (Self::Header, Self::Body);
}

impl<T> Block for alloy_consensus::Block<T>
where
    T: SignedTransaction,
{
    type Header = Header;
    type Body = alloy_consensus::BlockBody<T>;

    fn new(header: Self::Header, body: Self::Body) -> Self {
        Self { header, body }
    }

    fn header(&self) -> &Self::Header {
        &self.header
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn split(self) -> (Self::Header, Self::Body) {
        (self.header, self.body)
    }
}

/// An extension trait for [`Block`]s that allows for mutable access to the block's internals.
///
/// This allows for modifying the block's header and body for testing purposes.
#[cfg(any(test, feature = "test-utils"))]
pub trait TestBlock: Block<Header: crate::test_utils::TestHeader> {
    /// Returns mutable reference to block body.
    fn body_mut(&mut self) -> &mut Self::Body;

    /// Returns mutable reference to block header.
    fn header_mut(&mut self) -> &mut Self::Header;

    /// Updates the block header.
    fn set_header(&mut self, header: Self::Header);

    /// Updates the parent block hash.
    fn set_parent_hash(&mut self, hash: alloy_primitives::BlockHash) {
        crate::header::test_utils::TestHeader::set_parent_hash(self.header_mut(), hash);
    }

    /// Updates the block number.
    fn set_block_number(&mut self, number: alloy_primitives::BlockNumber) {
        crate::header::test_utils::TestHeader::set_block_number(self.header_mut(), number);
    }

    /// Updates the block state root.
    fn set_state_root(&mut self, state_root: alloy_primitives::B256) {
        crate::header::test_utils::TestHeader::set_state_root(self.header_mut(), state_root);
    }

    /// Updates the block difficulty.
    fn set_difficulty(&mut self, difficulty: alloy_primitives::U256) {
        crate::header::test_utils::TestHeader::set_difficulty(self.header_mut(), difficulty);
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl<T> TestBlock for alloy_consensus::Block<T>
where
    T: SignedTransaction,
{
    fn body_mut(&mut self) -> &mut Self::Body {
        &mut self.body
    }

    fn header_mut(&mut self) -> &mut Self::Header {
        &mut self.header
    }

    fn set_header(&mut self, header: Self::Header) {
        self.header = header
    }
}
