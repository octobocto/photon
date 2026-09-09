use std::{collections::HashSet, net::SocketAddr};

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use l2l_openapi::open_api;
use photon_types::{
    Address, Authorized, Block, BlockIndex, MempoolTx, MerkleRoot, OutPoint,
    Output, OutputContent, Pointed, PointedOutput, SpentOutput, Transaction,
    Txid, WithdrawalBundle,
    net::Peer,
    schema as photon_schema,
    wallet::{Balance, TransferDests},
};

use super::{GetBlockTemplateResponse, GetTransactionResponse, schema};

#[open_api(ref_schemas[
    Address, MerkleRoot, OutPoint, Output, OutputContent, Txid,
    schema::BitcoinTxid, photon_schema::BitcoinAddr,
    photon_schema::BitcoinOutPoint,
])]
#[rpc(client, server)]
pub trait Rpc {
    /// Get balance in sats
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "balance")]
    async fn balance(&self) -> RpcResult<Balance>;

    /// Connect a block for which a BMM request was included in the specified
    /// mainchain block. Returns `true` if it was accepted as the new tip.
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "connect_block")]
    async fn connect_block(
        &self,
        block: Block,
        #[open_api_method_arg(schema(
            PartialSchema = "photon_schema::BitcoinBlockHash"
        ))]
        main_block_hash: bitcoin::BlockHash,
    ) -> RpcResult<bool>;

    /// Connect to a peer
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "connect_peer")]
    async fn connect_peer(
        &self,
        #[open_api_method_arg(schema(
            PartialSchema = "photon_schema::SocketAddr"
        ))]
        addr: SocketAddr,
    ) -> RpcResult<()>;

    /// Deposit to address
    #[open_api_method(output_schema(PartialSchema = "schema::BitcoinTxid"))]
    #[method(name = "create_deposit")]
    async fn create_deposit(
        &self,
        address: Address,
        value_sats: u64,
        fee_sats: u64,
    ) -> RpcResult<bitcoin::Txid>;

    /// Create a tx that transfers funds to the specified address
    #[method(name = "create_transfer")]
    async fn create_transfer(
        &self,
        dest: Address,
        value_sats: u64,
        fee_sats: u64,
    ) -> RpcResult<Txid>;

    /// Create a tx that transfers funds to each address in `dests`,
    /// which maps an address to a value in sats
    #[method(name = "create_transfer_many")]
    async fn create_transfer_many(
        &self,
        dests: TransferDests,
        fee_sats: u64,
    ) -> RpcResult<Txid>;

    /// Creates a tx that initiates a withdrawal to the specified mainchain
    /// address
    #[method(name = "create_withdrawal")]
    async fn create_withdrawal(
        &self,
        #[open_api_method_arg(schema(
            PartialSchema = "photon_types::schema::BitcoinAddr"
        ))]
        mainchain_address: bitcoin::Address<
            bitcoin::address::NetworkUnchecked,
        >,
        amount_sats: u64,
        fee_sats: u64,
        mainchain_fee_sats: u64,
    ) -> RpcResult<Txid>;

    /// Format a deposit address
    #[method(name = "format_deposit_address")]
    async fn format_deposit_address(
        &self,
        address: Address,
    ) -> RpcResult<String>;

    /// Delete peer from known_peers DB.
    /// Connections to the peer are not terminated.
    #[method(name = "forget_peer")]
    async fn forget_peer(
        &self,
        #[open_api_method_arg(schema(
            PartialSchema = "photon_schema::SocketAddr"
        ))]
        addr: SocketAddr,
    ) -> RpcResult<()>;

    /// Generate a mnemonic seed phrase
    #[method(name = "generate_mnemonic")]
    async fn generate_mnemonic(&self) -> RpcResult<String>;

    /// Get the block with specified block hash, if it exists
    #[method(name = "get_block")]
    async fn get_block(
        &self,
        block_hash: photon_types::BlockHash,
    ) -> RpcResult<Option<photon_types::Block>>;

    /// Get the block hash at the specified height in the current chain,
    /// if it exists
    #[open_api_method(output_schema(
        PartialSchema = "schema::Optional<photon_types::BlockHash>"
    ))]
    #[method(name = "get_block_hash")]
    async fn get_block_hash(
        &self,
        height: u32,
    ) -> RpcResult<Option<photon_types::BlockHash>>;

    /// Get the transaction ids, sizes and encodings of a block, with the
    /// mainchain deposits and withdrawal bundle spends it applied
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "get_block_index")]
    async fn get_block_index(
        &self,
        block_hash: photon_types::BlockHash,
    ) -> RpcResult<BlockIndex>;

    /// Assemble a block to blind merge mine, without requesting BMM for it.
    /// The caller requests BMM for `critical_hash` itself, then passes the
    /// block back to `connect_block`.
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "get_block_template")]
    async fn get_block_template(&self) -> RpcResult<GetBlockTemplateResponse>;

    /// Get mainchain blocks that commit to a specified block hash
    #[open_api_method(output_schema(
        PartialSchema = "photon_schema::BitcoinBlockHash"
    ))]
    #[method(name = "get_bmm_inclusions")]
    async fn get_bmm_inclusions(
        &self,
        block_hash: photon_types::BlockHash,
    ) -> RpcResult<Vec<bitcoin::BlockHash>>;

    /// Get the best mainchain block hash known by Photon
    #[open_api_method(output_schema(
        PartialSchema = "schema::Optional<photon_schema::BitcoinBlockHash>"
    ))]
    #[method(name = "get_best_mainchain_block_hash")]
    async fn get_best_mainchain_block_hash(
        &self,
    ) -> RpcResult<Option<bitcoin::BlockHash>>;

    /// Get the best sidechain block hash known by Photon
    #[open_api_method(output_schema(
        PartialSchema = "schema::Optional<photon_types::BlockHash>"
    ))]
    #[method(name = "get_best_sidechain_block_hash")]
    async fn get_best_sidechain_block_hash(
        &self,
    ) -> RpcResult<Option<photon_types::BlockHash>>;

    /// Get a new address
    #[method(name = "get_new_address")]
    async fn get_new_address(&self) -> RpcResult<Address>;

    /// Get stxos for addresses
    #[method(name = "get_stxos")]
    async fn get_stxos(
        &self,
        addresses: HashSet<Address>,
    ) -> RpcResult<Vec<Pointed<SpentOutput>>>;

    /// Get transaction by txid
    #[method(name = "get_transaction")]
    async fn get_transaction(
        &self,
        txid: Txid,
    ) -> RpcResult<Option<GetTransactionResponse>>;

    /// Get utxos for addresses
    #[method(name = "get_utxos")]
    async fn get_utxos(
        &self,
        addresses: HashSet<Address>,
    ) -> RpcResult<Vec<PointedOutput>>;

    /// Get wallet addresses, sorted by base58 encoding
    #[method(name = "get_wallet_addresses")]
    async fn get_wallet_addresses(&self) -> RpcResult<Vec<Address>>;

    /// Get wallet UTXOs
    #[method(name = "get_wallet_utxos")]
    async fn get_wallet_utxos(&self) -> RpcResult<Vec<PointedOutput>>;

    /// Get the current block count
    #[method(name = "getblockcount")]
    async fn getblockcount(&self) -> RpcResult<u32>;

    /// Get the height of the latest failed withdrawal bundle
    #[method(name = "latest_failed_withdrawal_bundle_height")]
    async fn latest_failed_withdrawal_bundle_height(
        &self,
    ) -> RpcResult<Option<u32>>;

    /// List the transactions the mempool holds, in no particular order.
    #[method(name = "list_mempool")]
    async fn list_mempool(&self) -> RpcResult<Vec<MempoolTx>>;

    /// List peers
    #[method(name = "list_peers")]
    async fn list_peers(&self) -> RpcResult<Vec<Peer>>;

    /// List all UTXOs
    #[method(name = "list_utxos")]
    async fn list_utxos(&self) -> RpcResult<Vec<PointedOutput>>;

    /// Attempt to mine a sidechain block
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "mine")]
    async fn mine(&self, fee: Option<u64>) -> RpcResult<()>;

    /// Get OpenAPI schema
    #[open_api_method(output_schema(PartialSchema = "schema::OpenApi"))]
    #[method(name = "openapi_schema")]
    async fn openapi_schema(&self) -> RpcResult<utoipa::openapi::OpenApi>;

    /// Get pending withdrawal bundle
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "pending_withdrawal_bundle")]
    async fn pending_withdrawal_bundle(
        &self,
    ) -> RpcResult<Option<WithdrawalBundle>>;

    /// Remove a tx from the mempool
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "remove_from_mempool")]
    async fn remove_from_mempool(&self, txid: Txid) -> RpcResult<()>;

    /// Set the wallet seed from a mnemonic seed phrase
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "set_seed_from_mnemonic")]
    async fn set_seed_from_mnemonic(&self, mnemonic: String) -> RpcResult<()>;

    /// Get total sidechain wealth
    #[method(name = "sidechain_wealth")]
    async fn sidechain_wealth_sats(&self) -> RpcResult<u64>;

    /// Sign a transaction, and optionally broadcast it.
    #[method(name = "sign_transaction")]
    async fn sign_transaction(
        &self,
        transaction: Transaction,
        broadcast: Option<bool>,
    ) -> RpcResult<Authorized<Transaction>>;

    /// Verify and broadcast a transaction
    #[method(name = "submit_transaction")]
    async fn submit_transaction(
        &self,
        transaction: Authorized<Transaction>,
    ) -> RpcResult<Txid>;

    /// Stop the node
    #[method(name = "stop")]
    async fn stop(&self);
}
