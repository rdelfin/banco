use banco_interface::node::{
    node_server::{Node, NodeServer},
    NodeStatus, StatusRequest, StatusResponse,
};

pub trait NodeInterface {
    fn is_ok() -> bool;
}
