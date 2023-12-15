//! Executable and library for Process Control Hyperdrive MQTT project.
//! 
//! The project folder is divided into two parts:
//! * client: 
//! Different client implementations, each with their own thread, that subscribe to topics and publish messages.
//! 
//! * library: 
//! Includes the wrapper API around rumqttc, an util module with small helper functions, and two enums holding topics and payloads. 
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

mod client;
mod library;

use crate::{library::*, client::*};
use std::{thread, time::Duration};

fn main() {
    let vehicle_list: Vec<String> = vec![
        String::from("f2e85f2f5770"),
        //String::from("d98ebab7c206"),
        //String::from("c60ee9d05225"),
        //String::from("c40caf091413"),
        //String::from("d11d2fea5c74"),
    ];

    // Shared MQTT client for helper function such as discover_vehicles, connect_vehicles, etc.
    let (mut client, connection) = Mqtt::new("groupg_main");
    // Channel receiver to receive messages from a connection loop. This specific one is only used by the discover_vehicles function.
    let rx = connection.start_loop();

    // Discover and print vehicles IDs if none are specified
    if vehicle_list.is_empty() {
        discover_vehicles(&mut client, &rx);
    }

    // Start relay first to avoid lost connect messages
    let _relay = Relay::new(&vehicle_list).run();
    thread::sleep(Duration::from_millis(30)); // Hack for lost connect messages (TODO)

    // Connect to vehicles and start the clients
    connect_vehicles(&mut client, &vehicle_list);
    let _blink = Blink::new(&vehicle_list).run();
    let _speed = Speed::new(&[300, 400, 500], &vehicle_list).run();
    let _lane = Lane::new(&[0, 10, 0, -10], &vehicle_list).run();
    let _track = Track::new(&vehicle_list, &[20, 4, 21]).run();

    // CTRL+C handler to disconnect vehicles on exit
    set_ctrlc_handler(&client, &vehicle_list);

    // Block thread and publish emergency messages on keypresses of enter
    blocking_emergency_handler(&mut client);
}

