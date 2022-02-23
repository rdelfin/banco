use crate::nodes::NodeManager;
use banco_interfaces::{ListNodesResponse, Node, Result, Teller};
use std::{collections::HashMap, sync::Arc};
use tarpc::{
    context::Context,
    serde_transport,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::UnixListener;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;

mod nodes;

#[derive(Clone)]
struct TellerServer {
    node_manager: Arc<NodeManager>,
}

#[tarpc::server]
impl Teller for TellerServer {
    async fn list_nodes(self, _: Context) -> ListNodesResponse {
        ListNodesResponse {
            nodes: HashMap::new(),
        }
    }

    async fn start_node(self, _: Context, node: Node) -> Result {
        self.node_manager.start_node(node).await
    }

    async fn heartbeat(self, _: Context, _node_name: String) {}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = run_server().await;
    if let Err(e) = std::fs::remove_file("/var/run/banco/teller.sock") {
        println!("There was an error removing the socket file: {:?}", e);
    }
    res
}

async fn run_server() -> anyhow::Result<()> {
    let node_manager = Arc::new(NodeManager::new());

    let addr = "/var/run/banco/teller.sock";
    let listener = UnixListener::bind(addr)?;
    let codec_builder = LengthDelimitedCodec::builder();
    tokio::spawn(async move {
        loop {
            let node_manager = node_manager.clone();
            let (conn, _addr) = listener.accept().await.unwrap();
            let framed = codec_builder.new_framed(conn);
            let transport = serde_transport::new(framed, Bincode::default());

            let req = BaseChannel::with_defaults(transport)
                .execute(TellerServer { node_manager }.serve());
            tokio::spawn(req);
        }
    })
    .await?;
    Ok(())
}
