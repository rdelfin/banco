use banco_interfaces::{ListNodesResponse, Node, Teller};
use std::collections::HashMap;
use tarpc::{
    context::Context,
    serde_transport,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

#[derive(Clone)]
struct TellerServer;

#[tarpc::server]
impl Teller for TellerServer {
    async fn list_nodes(self, _: Context) -> ListNodesResponse {
        ListNodesResponse {
            nodes: HashMap::new(),
        }
    }

    async fn start_node(self, _: Context, _node: Node) {}

    async fn heartbeat(self, _: Context, _node_name: String) {}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "/var/run/banco/teller.sock";
    let listener = UnixListener::bind(addr)?;
    let codec_builder = LengthDelimitedCodec::builder();
    tokio::spawn(async move {
        loop {
            let (conn, _addr) = listener.accept().await.unwrap();
            let framed = codec_builder.new_framed(conn);
            let transport = serde_transport::new(framed, Bincode::default());

            let fut = BaseChannel::with_defaults(transport).execute(TellerServer.serve());
            tokio::spawn(fut);
        }
    })
    .await?;
    Ok(())
}
