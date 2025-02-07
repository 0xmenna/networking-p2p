use std::{error::Error, time::Duration};

use clap::{command, Parser};
use codec::{Decode, Encode};
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Swarm};
use log::{debug, info};
use networking::{
    behaviour::{
        base::{BaseBehaviour, BaseBehaviourEvent},
        wrapped::Wrapped,
    },
    builder::P2PTransportBuilder,
    cli::TransportArgs,
    AgentInfo,
};
use tokio::time;

#[derive(Parser)]
#[command(version, author)]
struct Cli {
    #[command(flatten)]
    pub transport: TransportArgs,
}

//
// Dummy definitions for demonstration purposes.
// In your actual code these should be replaced by your real definitions.
//
#[derive(Debug, Clone, Encode, Decode)]
struct Block {
    hash: u64,
    data: Vec<u8>,
}

//
// Example main
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger (env_logger is a popular choice)
    env_logger::init();

    let cli = Cli::parse();
    let agent_info = get_agent_info!();

    // Build the transport builder from CLI arguments.
    let builder = P2PTransportBuilder::from_cli(cli.transport, agent_info).await?;

    // build_node() now creates a Swarm with your NodeBehaviour.
    let mut swarm: Swarm<Wrapped<BaseBehaviour>> = builder.build_swarm(|base| base.into())?;

    // Subscribe to the "blocks" topic via the underlying pubsub.
    // Assuming your NodeBehaviour exposes an inner BaseBehaviour
    // with a subscribe method.
    swarm.behaviour_mut().subscribe("blocks");

    // Create an interval timer to publish blocks periodically.
    let mut publish_interval = time::interval(Duration::from_secs(5));
    let mut block_number: u64 = 0;

    info!("Starting event loop");

    loop {
        tokio::select! {
            // Every 5 seconds, create and publish a new block.
            _ = publish_interval.tick() => {
                block_number += 1;
                let block = Block {
                    hash: block_number,
                    data: format!("Block number {}", block_number).into_bytes(),
                };

                info!("Publishing block: {:?}", block);
                // Publish the block on the "blocks" topic.
                swarm.behaviour_mut().publish_message("blocks", block);
            },

            // Process events from the swarm.
            event = swarm.select_next_some() => {
                match event {
                    // When a pubsub message event is received, handle it.
                    SwarmEvent::Behaviour(BaseBehaviourEvent::Gossipsub(msg)) => {
                        info!("Received block message on topic '{}': {:?}", msg.topic, msg.message);
                    },
                    // Handle other swarm events if needed.
                    other => {
                        debug!("Other swarm event: {:?}", other);
                    }
                }
            }
        }
    }
}

mod networking;
