use jsonrpc_derive::rpc;
use jsonrpc_core::{IoHandler, Result, BoxFuture};
use jsonrpc_core::futures::future;
use jsonrpc_http_server::ServerBuilder;
use std::net::SocketAddr;

#[rpc(server)]
pub trait Rpc {
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "add")]
    fn add(&self, a: u64, b: u64) -> Result<u64>;

    #[rpc(name = "callAsync")]
    fn call(&self, a: u64) -> BoxFuture<Result<String>>;
}

struct RpcImpl;
impl Rpc for RpcImpl {
    fn protocol_version(&self) -> Result<String> {
        Ok("version1".into())
    }

    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }

    fn call(&self, _: u64) -> BoxFuture<Result<String>> {
        Box::pin(future::ready(Ok("OK".to_owned()).into()))
    }
}

fn main() {
    let mut io = IoHandler::new();
    let rpc = RpcImpl;

    io.extend_with(rpc.to_delegate());
    let addr =  SocketAddr::new("0.0.0.0".parse().unwrap(), 5000);
    let server = ServerBuilder::new(io)
        .threads(5)
        .start_http(&addr)
        .unwrap();
    server.wait();
}