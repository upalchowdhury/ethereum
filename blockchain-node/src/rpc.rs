use jsonrpsee::{
    core::{async_trait, server::rpc_module::Methods, Error},
    http_server::{HttpServerBuilder, HttpServerHandle},
    server::{ServerBuilder, ServerHandle},
    RpcModule,
    proc_macros::rpc,
};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};


// Start JsonRPC server and register ethereum RPC mehtods to it
// TODO implement database
pub struct Rpc<DB: Database> {
    node: Arc<Node<DB>>,
    handle: Option<HttpServerHandle>,
    address: SocketAddr,
}