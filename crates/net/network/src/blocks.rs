use crate::message::NewBlockMessage;
use reth_eth_wire::NewBlockHashes;
use reth_primitives::PeerId;

/// All events related to blocks emitted by the network.
#[derive(Debug)]
pub enum NetworkBlockEvent {
    /// Represents the event of receiving a block from a peer.
    ///
    /// This indicates a block that was broadcasted to us from the peer.
    IncomingNewBlockMessage {
        /// The ID of the peer from which the block were received.
        peer_id: PeerId,
        /// The received block.
        msg: NewBlockMessage,
    },
    /// Represents the event of receiving a list of block hashes from a peer.
    IncomingNewBlockHashes {
        /// The ID of the peer from which the block hashes were received.
        peer_id: PeerId,
        /// The received new block hashes.
        msg: NewBlockHashes,
    },
}
