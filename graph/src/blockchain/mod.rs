//! The `blockchain` module exports the necessary traits and data structures to integrate a
//! blockchain into Graph Node. A blockchain is represented by an implementation of the `Blockchain`
//! trait which is the centerpiece of this module.

pub use anyhow::Error;

use crate::prelude::EthereumBlockPointer;
use std::sync::Arc;

pub type BlockPtr = EthereumBlockPointer;

trait Blockchain: Sized + Send + Sync + 'static {
    type Block: Block;
    // type DataSource: DataSource<Self>;
    type DataSourceTemplate;
    // type Manifest<Self>;
    // type IngestorAdapter: IngestorAdapter<Self>;
    // type TriggersAdapter: TriggersAdapter<Self>;
    // type BlockStream: BlockStream<Self>;

    // Trigger data as parsed from the triggers adapter.
    type TriggerData;

    // Decoded trigger ready to be processed by the mapping.
    type MappingTrigger: AscType;
    // type TriggerFilter: TriggerFilter<Self>;
    type NodeCapabilities;
    // type RuntimeAdapter: RuntimeAdapter;
    // ...WIP

    fn reorg_threshold() -> u32;
    fn triggers_adapter(
        &self,
        network: &str,
        capabilities: Self::NodeCapabilities,
    ) -> Arc<Self::TriggersAdapter>;

    fn new_block_stream(
        &self,
        current_head: BlockPtr,
        filter: Self::TriggerFilter,
    ) -> Result<Self::BlockStream, Error>;
}

trait Block {
    fn ptr(&self) -> BlockPtr;
    fn parent_ptr(&self) -> Option<BlockPtr>;

    fn number(&self) -> u64 {
        self.ptr().number
    }

    fn hash(&self) -> Box<[u8]> {
        self.ptr().hash
    }
}
