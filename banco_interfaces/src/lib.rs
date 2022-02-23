//! This module exposes all the interfaces used by banco, both from nodes, and teller. Use these to
//! communicate to other components over IPC.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

mod error;

pub use crate::error::{Error, Result};

/// This interface is used to talk to Teller, the broker and manager for all nodes and topics.
#[tarpc::service]
pub trait Teller {
    /// List all registered nodes.
    async fn list_nodes() -> ListNodesResponse;
    /// Request teller to start a node. It will create a process, and return once the `fork`
    /// succeeeded.
    async fn start_node(node: Node) -> Result;

    /// Endpoint used for the nodes to heartbeat back to teller.
    async fn heartbeat(node_name: String);
}

/// Response to listing of all nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNodesResponse {
    /// Hashmap of said nodes
    pub nodes: HashMap<String, Node>,
}

/// A representation of the current state of a node from the view of Teller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// The registered name of the node. It will remain consistent throughout a run.
    pub name: String,
    /// The path to the executable used for the binary.
    pub executable_path: PathBuf,
    /// The status the node is in (e.g. running, stopped).
    pub status: NodeStatus,
}

/// Enum representing the current state of a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Used once the node has been created, it is identified as alive, but hasn't finished its
    /// internally-defined initialisation.
    Initialising,
    /// Used once the node is running and in a good state
    Running,
    /// Used for when a node is no longer running, be it intentionally or after a crash.
    Stopped { crashed: bool },
}
