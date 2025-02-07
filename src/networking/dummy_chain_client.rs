use libp2p::{futures::Stream, PeerId};
use std::{collections::HashSet, pin::Pin};

pub type ClientError = ();

pub type AuthorityPeers = HashSet<PeerId>;

pub type NodeStream =
    Pin<Box<dyn Stream<Item = Result<AuthorityPeers, ClientError>> + Send + 'static>>;

pub struct ContractClient;

impl Default for ContractClient {
    fn default() -> Self {
        Self
    }
}

impl ContractClient {
    pub fn network_nodes_stream(&self, _interval: std::time::Duration) -> NodeStream {
        // return baseed on get_authorities
        Box::pin(futures::stream::once(async { Ok(get_authority_peers()) }))
    }
}

fn get_authority_peers() -> AuthorityPeers {
    // Just add a single random peer
    let mut authorities = HashSet::new();
    authorities.insert(PeerId::random());

    authorities
}
