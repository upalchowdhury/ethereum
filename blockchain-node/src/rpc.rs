use jsonrpsee::{
    server::{ServerBuilder, ServerHandle},
    RpcModule,
};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};


// Start JsonRPC server and register ethereum RPC mehtods to it

