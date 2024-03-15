use jsonrpsee::{
    core::{async_trait, server::rpc_module::Methods, Error},
    http_server::{HttpServerBuilder, HttpServerHandle},
    server::{ServerBuilder, ServerHandle},
    RpcModule,
    proc_macros::rpc,
};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};
use database::*;
use eth-types::block::BlockTag;


// Start JsonRPC server and register ethereum RPC mehtods to it
// TODO implement database
pub struct Rpc<DB: Storage> {
    node: Arc<Node<DB>>,
    handle: Option<HttpServerHandle>,
    address: SocketAddr,
}


trait EthRpc {
    
    async fn get_balance(&self, address: &str, block: BlockTag) -> Result<String, Error>;
  
    async fn get_transaction_count(&self, address: &str, block: BlockTag) -> Result<String, Error>;

    async fn get_block_transaction_count_by_hash(&self, hash: H256) -> Result<String, Error>;


}

#[async_trait]
impl<DB: Storage> EthRpc for Rpc<DB> {
    async fn get_balance(&self, address: &str, block: BlockTag) -> Result<String, Error> {
        let address = convert_err(Address::from_str(address))?;
        let balance = convert_err(self.node.get_balance(&address, block).await)?;

        Ok(format_hex(&balance))
    }

    async fn get_transaction_count(&self, address: &str, block: BlockTag) -> Result<String, Error> {
        let address = convert_err(Address::from_str(address))?;
        let nonce = convert_err(self.node.get_nonce(&address, block).await)?;

        Ok(format!("0x{nonce:x}"))
    }

    async fn get_block_transaction_count_by_hash(&self, hash: H256) -> Result<String, Error> {
        let transaction_count =
            convert_err(self.node.get_block_transaction_count_by_hash(&hash).await)?;
        Ok(u64_to_hex_string(transaction_count))
    }

}