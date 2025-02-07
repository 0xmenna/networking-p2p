use std::fmt::{Display, Formatter};

use libp2p::{noise, swarm::DialError, TransportError};
use serde::{Deserialize, Serialize};

pub mod behaviour;
pub mod builder;
pub mod chain_client;
pub mod cli;
pub mod protocol;
pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Libp2p transport creation failed: {0}")]
    Transport(String),
    #[error("Listening failed: {0:?}")]
    Listen(#[from] TransportError<std::io::Error>),
    #[error("Dialing failed: {0:?}")]
    Dial(#[from] DialError),
    #[error("Decoding message failed: {0}")]
    Decode(String),
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
