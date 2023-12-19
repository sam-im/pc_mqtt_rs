//! Process Control Hyperdrive MQTT project (Rust version)
//!
//! This documentation can either be read from the source code or by using the "doc/index.html" (recommended). The latter was generated using "cargo doc" and can be found in the "doc" folder.
//! 
//! [Video demonstration link - Google Drive](https://drive.google.com/file/d/1ivpBfDTXD7pe8Fv8COzepab6ASXwm5Hu/view?usp=sharing)
//!
//! The project consists of 4 parts:
//! * Steering controller (blink, speed, lane)
//! * Emergency controller (relay)
//! * Track the tracks controller (track, relay)
//! * Personal addition (track, relay)
//!
//! All client implementations is found within the client module and shared code is found within the library module.
//!
//! # Available controllers/clients
//! Each client module has a struct that holds some data about its purpose and a vehicle list. They all initialize a new MQTT client and run in their own thread.
//! Since all the communication is done through MQTT, they can be mixed and matched with their counterparts written in Python.
//! The only limitation being that the relay client needs to be started before any other client, to avoid lost connect messages.
//!
//! ## Steering controllers
//! ### Blink
//! Every second it sends a message to each vehicle setting their lights to either on or off.
//!
//! ### Speed
//! Every 3 seconds it sends a message to set the speed of each vehicle. The speed values are given to the controller with a list, and is iterated.
//!
//! ### Lane
//! Every 5 seconds it sends a message to change lane of each vehicle by iterating a list of values given as an argument.
//!
//! ## Emergency controller
//! Both the emergency and personal addition controllers are implemented inside the relay module/client.
//! The relay client is responsible for relaying messages from every other client to the broker. It will also handle emergency messages and personal addition (zone) messages, and if necessary overwrite any speed messages.
//!
//! ## Tracking and personal addition controllers
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
