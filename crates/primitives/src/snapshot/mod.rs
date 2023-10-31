//! Snapshot primitives.

mod compression;
mod filters;
mod segment;

use alloy_primitives::BlockNumber;
pub use compression::Compression;
pub use filters::{Filters, InclusionFilter, PerfectHashingFunction};
pub use segment::{SegmentHeader, SnapshotSegment};

/// Default snapshot block count.
pub const BLOCKS_PER_SNAPSHOT: u64 = 500_000;

/// Highest snapshotted block numbers, per data part.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct HighestSnapshots {
    /// Highest snapshotted block of headers, inclusive.
    /// If [`None`], no snapshot is available.
    pub headers: Option<BlockNumber>,
    /// Highest snapshotted block of receipts, inclusive.
    /// If [`None`], no snapshot is available.
    pub receipts: Option<BlockNumber>,
    /// Highest snapshotted block of transactions, inclusive.
    /// If [`None`], no snapshot is available.
    pub transactions: Option<BlockNumber>,
}
