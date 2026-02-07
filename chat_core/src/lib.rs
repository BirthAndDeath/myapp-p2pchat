use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    str::FromStr,
    time::Duration,
};

use futures::stream::StreamExt;
use libp2p::{
    Swarm,
    futures::io,
    gossipsub, mdns, noise,
    swarm::{self, NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use tokio::sync::mpsc;
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

pub mod storage;
pub struct CoreConfig {
    ///example :"sqlite:///path/to/database.db"
    database_path: String,
}
impl CoreConfig {
    pub fn new(database_path: impl Into<std::string::String>) -> Self {
        Self {
            database_path: database_path.into(),
        }
    }
}
pub enum MessageEvent {
    newmassage,
}
pub struct ChatMeassage {
    pub event: MessageEvent,
    pub data: String,
}
pub struct ChatCore {
    pub swarm: Swarm<MyBehaviour>,
    pub topic: gossipsub::IdentTopic,
    pub tx_message: tokio::sync::mpsc::Sender<ChatMeassage>,
    pub rx_message: Option<tokio::sync::mpsc::Receiver<ChatMeassage>>,
}
impl ChatCore {
    pub fn try_init(cfg: &CoreConfig) -> anyhow::Result<Self> {
        storage::init(cfg)?;
        let mut swarm = swarm_init()?;
        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new("test-net");
        // subscribes to our topic
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        let (tx, rx) = mpsc::channel(32);

        Ok(ChatCore {
            swarm,
            tx_message: tx,
            rx_message: Some(rx),
            topic,
        })
    }
    pub fn sendmessage(&mut self, data: String) {
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.topic.clone(), data.as_bytes())
        {
            println!("Publish error: {e:?}");
        }
    }
    fn sendmessage_mpsc(&mut self, data: String) {
        let message = ChatMeassage {
            event: MessageEvent::newmassage,
            data,
        };
        let tx = self.tx_message.clone();
        tokio::spawn(async move {
            tx.send(message)
                .await
                .expect("falied send message:tx_message ");
        });
    }
}
fn swarm_init() -> anyhow::Result<Swarm<MyBehaviour>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message
                // signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .build()
                .map_err(io::Error::other)?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .build();

    Ok(swarm)
}
pub fn swarm_event(event: SwarmEvent<MyBehaviourEvent>, core: &mut ChatCore) {
    match event {
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                core.sendmessage_mpsc(format!("mDNS discovered a new peer: {peer_id}"));

                core.swarm
                    .behaviour_mut()
                    .gossipsub
                    .add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                core.sendmessage_mpsc(format!("mDNS discover peer has expired: {peer_id}"));
                core.swarm
                    .behaviour_mut()
                    .gossipsub
                    .remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        })) => {
            core.sendmessage_mpsc(format!(
                "Got message: '{}' with id: {id} from peer: {peer_id}",
                String::from_utf8_lossy(&message.data)
            ));
        }
        SwarmEvent::NewListenAddr { address, .. } => {
            core.sendmessage_mpsc(format!("Local node is listening on {address}"));
        }
        _ => {}
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
