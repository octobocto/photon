//! RPC API

use photon_types::{Block, BlockHash, Transaction};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod schema;

#[allow(clippy::double_must_use)]
mod rpc;

pub use rpc::{RpcClient, RpcDoc, RpcServer};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct GetBlockTemplateResponse {
    /// Block hash to commit to in a BMM request
    pub critical_hash: BlockHash,
    /// Block to pass to `connect_block` once its BMM request is included in a
    /// mainchain block
    pub block: Block,
    /// Fees collected by the transactions in the block, in sats
    pub fees_sats: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct GetTransactionResponse {
    pub tx: Transaction,
    /// Block hash, if in the active chain
    pub block_hash: Option<BlockHash>,
}
