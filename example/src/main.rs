use clap::{command, Parser};
use codec::{Decode, Encode};
use env_logger::Env;
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
    protocol::BLOCKS_TOPIC,
    AgentInfo,
};
use std::{error::Error, time::Duration};
use tokio::time;

#[derive(Parser)]
#[command(version, author)]
struct Cli {
    #[command(flatten)]
    pub transport: TransportArgs,

    #[clap(long, default_value = "false")]
    pub block_producer: bool,

    #[clap(long, default_value = "")]
    pub name: String,
}

//
// A dummy representation of a block.
//
#[derive(Debug, Clone, Encode, Decode)]
struct Block {
    block_num: u64,
    data: Vec<u8>,
}

//
// A dummy representation vote on a block.
//
#[derive(Debug, Clone, Encode, Decode)]
struct Vote {
    voter: String,
    block: Block,
}

//
// A message sent over the network.
//
#[derive(Debug, Clone, Encode, Decode)]
enum Message {
    BlockProposal(Block),
    Vote(Vote),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    let agent_info = networking::get_agent_info!();

    // Build the transport builder from CLI arguments.
    let builder = P2PTransportBuilder::from_cli(cli.transport, agent_info).await?;

    // Create a default Swarm.
    let mut swarm = builder.build_default_swarm()?;

    if cli.block_producer {
        start_block_producer(&mut swarm).await?;
    } else {
        start_voter(&mut swarm, &cli.name).await?;
    }

    unreachable!();
}

pub async fn start_block_producer(
    swarm: &mut Swarm<Wrapped<BaseBehaviour>>,
) -> Result<(), Box<dyn Error>> {
    // Subscribe to the to the blocks topic.
    swarm.behaviour_mut().subscribe(BLOCKS_TOPIC);

    // Interval timer to publish blocks periodically.
    let mut publish_interval = time::interval(Duration::from_secs(5));
    let mut block_number: u64 = 0;

    loop {
        tokio::select! {
            // Every 5 seconds, create and publish a new block.
            _ = publish_interval.tick() => {
                block_number += 1;
                let block = Block {
                    block_num: block_number,
                    data: format!("Block number {}", block_number).into_bytes(),
                };

                // Publish the block.
                info!("Publishing block: {:?}", block);
                swarm.behaviour_mut().publish_message(BLOCKS_TOPIC, Message::BlockProposal(block));
            },

            // Process events from the swarm.
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BaseBehaviourEvent::Gossipsub(msg)) => {
                        let message = Message::decode(&mut &msg.message[..]).unwrap();
                        match message {
                            Message::Vote(vote) => {
                              info!("--------------------------------------------");
                              info!("Message from Peer: {:?}", msg.peer_id);

                              info!("Received vote: {:?}", vote)},
                            _ => {},
                        }
                    },
                    other => {
                        debug!("Other swarm event: {:?}", other);
                    }
                }
            }
        }
    }
}

pub async fn start_voter(
    swarm: &mut Swarm<Wrapped<BaseBehaviour>>,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    swarm.behaviour_mut().subscribe(BLOCKS_TOPIC);

    loop {
        tokio::select! {
            // Process events from the swarm.
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BaseBehaviourEvent::Gossipsub(msg)) => {
                        let message = Message::decode(&mut &msg.message[..]).unwrap();
                        match message {
                            Message::BlockProposal(block) => {
                              info!("--------------------------------------------");
                              info!("Message from Peer: {:?}", msg.peer_id);

                              info!("Received block: {:?}", block);
                              // Create a vote for the received block.
                              let vote = Vote {
                                    voter: name.to_string(),
                                    block: block,
                              };

                              info!("Publishing vote: {:?}", vote);
                              swarm.behaviour_mut().publish_message(BLOCKS_TOPIC, Message::Vote(vote));
                            },
                            _ => {},
                        }
                    },
                    other => {
                        debug!("Other swarm event: {:?}", other);
                    }
                }
            }
        }
    }
}
