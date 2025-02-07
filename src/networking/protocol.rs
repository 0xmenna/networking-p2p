use clap::ValueEnum;
use libp2p::StreamProtocol;

pub const BLOCK_PROPOSAL_TOPIC: &str = "/iceberg/block_proposal/1.0.0";
pub const BLOCK_VOTING_TOPIC: &str = "/iceberg/block_voting/1.0.0";

pub const ID_PROTOCOL: &str = "/iceberg/1.0.0";

pub const MAX_PUBSUB_MSG_SIZE: usize = 65536;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
#[clap(rename_all = "kebab_case")]
pub enum Network {
    Testnet,
    #[default]
    Mainnet,
}

pub const KNOWN_TOPICS: [&'static str; 2] = [BLOCK_PROPOSAL_TOPIC, BLOCK_VOTING_TOPIC];

pub const fn dht_protocol(network: Network) -> StreamProtocol {
    match network {
        Network::Testnet => StreamProtocol::new("/iceberg/dht/testnet/1.0.0"),
        Network::Mainnet => StreamProtocol::new("/iceberg/dht/mainnet/1.0.0"),
    }
}
