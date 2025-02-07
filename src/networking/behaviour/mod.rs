use base::BaseBehaviour;
use libp2p_swarm_derive::NetworkBehaviour;
use wrapped::Wrapped;

pub mod addr_cache;
pub mod base;
pub mod pubsub;
pub mod whitelist;
pub mod wrapped;

#[derive(NetworkBehaviour)]
pub struct NodeBehaviour {
    pub inner: Wrapped<BaseBehaviour>,
}
