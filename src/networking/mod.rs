use std::fmt::{Display, Formatter};

use libp2p::{noise, swarm::DialError, TransportError};
use serde::{Deserialize, Serialize};
use utils::parse_env_var;

pub mod behaviour;
pub mod builder;
pub mod cli;
pub mod dummy_chain_client;
pub mod protocol;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuicConfig {
    /// Maximum transmission unit to use during MTU discovery (default: 1452).
    pub mtu_discovery_max: u16,
    /// Interval for sending keep-alive packets in milliseconds (default: 5000).
    pub keep_alive_interval_ms: u32,
    /// Timeout after which idle connections are closed in milliseconds (default: 60000).
    pub max_idle_timeout_ms: u32,
}

impl QuicConfig {
    pub fn from_env() -> Self {
        let mtu_discovery_max = parse_env_var("MTU_DISCOVERY_MAX", 1452);
        let keep_alive_interval_ms = parse_env_var("KEEP_ALIVE_INTERVAL_MS", 5000);
        let max_idle_timeout_ms = parse_env_var("MAX_IDLE_TIMEOUT_MS", 60000);
        Self {
            mtu_discovery_max,
            keep_alive_interval_ms,
            max_idle_timeout_ms,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Libp2p transport creation failed: {0}")]
    Transport(String),
    #[error("Listening failed: {0:?}")]
    Listen(#[from] TransportError<std::io::Error>),
    #[error("Dialing failed: {0:?}")]
    Dial(#[from] DialError),
    // #[error("{0}")]
    // Contract(#[from] sqd_contract_client::ClientError),
}

impl From<noise::Error> for Error {
    fn from(e: noise::Error) -> Self {
        Self::Transport(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Transport(e.to_string())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: &'static str,
    pub version: &'static str,
}

impl Display for AgentInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.name, self.version)
    }
}

#[macro_export]
macro_rules! get_agent_info {
    () => {
        AgentInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    };
}
