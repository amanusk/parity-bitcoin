extern crate rustc_hex as hex;
extern crate heapsize;
extern crate primitives;
extern crate rayon;
extern crate bitcrypto as crypto;
extern crate serialization as ser;
#[macro_use]
extern crate serialization_derive;

pub mod constants;

mod block;
mod block_header;
mod merkle_root;
mod transaction;

/// `IndexedBlock` extension
mod read_and_hash;
mod indexed_block;
mod indexed_header;
mod indexed_transaction;

pub use primitives::{hash, bytes, bigint, compact};

pub use crate::block::Block;
pub use crate::block_header::BlockHeader;
pub use crate::merkle_root::{merkle_root, merkle_node_hash};
pub use crate::transaction::{Transaction, TransactionInput, TransactionOutput, OutPoint};

pub use crate::read_and_hash::{ReadAndHash, HashedData};
pub use crate::indexed_block::IndexedBlock;
pub use crate::indexed_header::IndexedBlockHeader;
pub use crate::indexed_transaction::IndexedTransaction;

pub type ShortTransactionID = hash::H48;
