use libp2p::{
    autonat, gossipsub, identify,
    kad::{self, store::MemoryStore},
    ping, relay,
};
use libp2p_swarm_derive::NetworkBehaviour;

use super::behaviour::{whitelist::WhitelistBehavior, wrapped::Wrapped};

#[derive(NetworkBehaviour)]
struct Behaviour {
    identify: identify::Behaviour,
    kademlia: kad::Behaviour<MemoryStore>,
    relay: relay::Behaviour,
    gossipsub: gossipsub::Behaviour,
    ping: ping::Behaviour,
    autonat: autonat::Behaviour,
    conn_limits: libp2p_connection_limits::Behaviour,
    whitelist: Wrapped<WhitelistBehavior>,
}
