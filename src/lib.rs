//! Process Control Hyperdrive MQTT project - Rust implementation
//! 
//! The project folder is divided into two parts:
//! * client: 
//! Different client implementations, each initialize a new MQTT client and runs in their own thread.
//! 
//! * library: 
//! Includes a wrapper API of rumqttc, utils for small helper functions, and two enums that holds the topic and payload templates. 
//! 
//! # Usage
//! 
//! # Clients
//! Each client implementation is a struct that holds necessary data about its purpose and a vehicle list to which it should connect. They all initialize a new MQTT client and run in their own thread.
//! 
//! ## Relay
//! The relay client is responsible for relaying messages from every other client to the broker. It also handles emergency messages and slow zone messages, by storing their state and if necessary overwriting speed messages before relaying them.
//! It needs to be started before any other client, to avoid lost connect messages.
//! 
//! ## Blink
//! Every second it will either turn on or off the lights of each vehicle.
//! 
//! ## Speed
//! Every 3 seconds it will send a speed message to each vehicle, with a speed value from a list of values.
//! 
//! ## Lane
//! Every 5 seconds it will change the lane of each vehicle, with a lane value from a list of values.
//! 
//! ## Track
//! This client keeps track of the current track ID for each vehicle. 
//! Whenever it receives a message from a vehicle with a track ID, it will store it check it against its list of slow tracks. If the track ID is in the list, it will add the vehicle ID to its list of slow vehicles and publish it to the broker.

mod library;
mod client;

pub use self::library::{
    mqtt::{Mqtt, ClientWrapper, ConnectionWrapper},
    topic::Topic,
    payload::Payload,
    util::{
        discover_vehicles,
        connect_vehicles,
        set_ctrlc_handler,
        blocking_emergency_handler,
    }
};

pub use self::client::{
    relay::Relay,
    blink::Blink,
    speed::Speed,
    lane::Lane,
    track::Track,
};
