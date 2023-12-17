//! Process Control Hyperdrive MQTT project (Rust version)
//!
//! The project src/ is divided into two parts:
//! * client/ that has different client implementations.
//! Each one of them initializes a new MQTT client and runs in their own thread.
//!
//! * library/ includes a wrapper API of rumqttc in mqtt.rs, small helper functions in util.rs, and two enums that holds topic and payload templates in topic.rs and payload.rs.
//!
//! # Available clients
//! Each client implementation is a struct that holds some data about useful for its purpose and a vehicle list. They all initialize a new MQTT client and run in their own thread.
//! Since all the communication is done through MQTT, they can be mixed and matched with their counterparts written in Python.
//!
//! ## Relay
//! The relay client is responsible for relaying messages from every other client to the broker. It will also handle emergency messages and zone messages, and if necessary overwrite any speed messages.
//! It needs to be started before any other client, to avoid lost connect messages.
//! ### Emergency (TODO)
//! ### Zone (TODO)
//!
//! ## Blink
//! Every second it sends a message to each vehicle controlling their lights, either on or off.
//!
//! ## Speed
//! Every 3 seconds it sends a speed message to each vehicle, by iterating a list of values given as an argument.
//!
//! ## Lane
//! Every 5 seconds it sends a message to change lane of each vehicle by iterating a list of values given as an argument.
//!
//! ## Track + Personal Addition
//! It receives and stores track ID numbers for each vehicle.
//! If a vehicle is on a track that has a speed limit (velocity 200), it is appended to "slow_vehicles" list and published for the relay to take action.

mod client;
mod library;

pub use self::library::{
    mqtt::{ClientWrapper, ConnectionWrapper, Mqtt},
    payload::Payload,
    topic::Topic,
    util::{blocking_emergency_handler, connect_vehicles, discover_vehicles, set_ctrlc_handler},
};

pub use self::client::{blink::Blink, lane::Lane, relay::Relay, speed::Speed, track::Track};
