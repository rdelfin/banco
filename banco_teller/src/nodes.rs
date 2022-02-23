use banco_interfaces::{Error, Node, NodeStatus, Result};
use std::collections::HashMap;
use tokio::{
    process::{Child, Command},
    sync::Mutex,
};

/// This class is used to manage data about nodes. This includes monitoring the status of nodes,
/// starting them, killing any errant ones, and everything in between.
pub struct NodeManager {
    nodes: Mutex<HashMap<String, (Node, Child)>>,
}

impl NodeManager {
    /// Creates a new node manager.
    pub fn new() -> NodeManager {
        NodeManager {
            nodes: Mutex::new(HashMap::new()),
        }
    }

    pub async fn start_node(&self, mut node: Node) -> Result {
        let mut nodes = self.nodes.lock().await;

        if nodes.contains_key(&node.name) {
            return Err(Error::NodeAlreadyExists);
        }

        let child = Command::new(&node.executable_path)
            .spawn()
            .map_err(Error::from_spawn)?;

        node.status = NodeStatus::Initialising;
        nodes.insert(node.name.clone(), (node, child));

        Ok(())
    }
}
