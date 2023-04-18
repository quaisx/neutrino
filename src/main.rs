/*
---------------- PROJECT NEUTRINO --------------------
 _______                 __         .__               
 \      \   ____  __ ___/  |________|__| ____   ____  
 /   |   \_/ __ \|  |  \   __\_  __ \  |/    \ /  _ \ 
/    |    \  ___/|  |  /|  |  |  | \/  |   |  (  <_> )
\____|__  /\___  >____/ |__|  |__|  |__|___|  /\____/ 
        \/     \/                           \/        
------------------ NO COPYRIGHTS ---------------------
-------------------- NO LICENSE ----------------------
-------------------- NO PATENTS ----------------------
------------------- FREE FOR ALL ---------------------
------------------- F*CK LAWYERS ---------------------
----------------- F*CK CAPITALISTM -------------------
@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */

 use libp2p::{
    swarm::{
        keep_alive, 
        NetworkBehaviour, 
        SwarmBuilder, 
        SwarmEvent
    },
    Multiaddr, 
    PeerId,
    ping,
    identity,
    identity::{
        Keypair
    }
};

use std::{error::Error};

use futures::prelude::*;

pub mod wallet;
pub mod utils;


#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

fn load_keypair() -> Keypair {
    // @TODO: check if this value has been stored, if so,
    // retrieve it, otherwise generate a new key pair,
    // store it locally, and return
    println!("@TODO: Add persistence to load_keypair");
    identity::Keypair::generate_ed25519()
}

/*
    We are starting off with a simple ping exchange 
 */
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!("NEUTRINO PING -> First step in building the app's p2p skeleton");

    // load this host's key pair
    let key_pair: Keypair = load_keypair();
    // host id is generated from the public key - pass the public key to generate host id
    let peer_id = PeerId::from(key_pair.public());
    println!("PEER ID: {peer_id:?}");
    
    // make sure features "full" is enabled in Cargo.tomp so that the compiler can see development_transport
    let transport = libp2p::development_transport(key_pair).await?;
    // need to create and join a swarm, start with a swarm builder
    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, Behaviour::default(), peer_id)
            .build();

    // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic".parse()?)?;
    // listen on all the interfaces - for now use a generic approach
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic".parse()?)?;

    // from the cli args pull another node's ip address and dial...
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("{peer_id} successfully dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("SwarmEvent::NewListenAddr: => {address:?}"),
            SwarmEvent::Behaviour(event) => println!("SwarmEvent::Behavior: => {event:?}"),
            _ => {}
        }
    }

}
