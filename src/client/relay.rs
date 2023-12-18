//! This module contains the Relay struct and its methods. It is responsible for relaying messages, as well as handling emergency and speed limit states.

use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use serde_json;
use std::thread::{self};

/// The Relay struct holds a list of vehicle IDs, the emergency state, a list of vehicles inside a slow zone, and the last speed value.
///
/// Everything except the vehicle list is updated by incoming messages.
pub struct Relay {
    vehicle_list: Vec<String>,
    emergency: bool,
    inside_slow_zone: Vec<String>,
    last_speed: i64,
}

impl Relay {
    /// Create a new Relay struct with a list of vehicle IDs.
    pub fn new(vehicle_list: &[String]) -> Relay {
        Relay {
            vehicle_list: vehicle_list.to_owned(),
            emergency: false,
            inside_slow_zone: Vec::new(),
            last_speed: 0,
        }
    }

    /// Handles all incoming messages and relays them to the correct recipient.
    ///
    /// This method is called in a loop, hence the name.
    ///
    /// The first thing it does is to create its own MQTT client and subscribe to the correct topics.
    ///
    /// Then it iterates over all incoming messages and checks if they are either Emergency ("GroupG/Emergency/I"), Zone ("GroupG/Zone/I") or Relay ("GroupG/Relay/") messages.
    ///
    /// Emergency and Zone messages are handled by updating the state of the Relay struct with the message payload's value.
    ///
    /// Relay messages are handled by either relaying them as is, or by selectively overwriting them with a new speed.
    fn loop_forever(mut self) {
        let (mut client, connection) = Mqtt::new("group-g_relay");
        client.subscribe(&Topic::Relay("#").get());
        client.subscribe(&Topic::Emergency.get());
        client.subscribe(&Topic::Zone.get());

        for message in connection.start_loop() {
            let payload_result = serde_json::from_slice(&message.payload);
            let payload: serde_json::Value = match payload_result {
                Ok(payload) => payload,
                Err(e) => {
                    dbg!(e);
                    continue;
                }
            };

            // Emergency messages handler
            if message.topic == Topic::Emergency.get() {
                self.emergency = match payload["payload"]["value"].as_bool() {
                    Some(value) => value,
                    None => {
                        dbg!("match returned None");
                        continue;
                    }
                };

                let speed = if self.emergency { 0 } else { 200 };

                for vehicle in &self.vehicle_list {
                    client.publish(
                        &Topic::VehicleI(vehicle).get(),
                        &Payload::Speed(speed, 1000).get(),
                    );
                }

                dbg!(&self.emergency);

            // Zone messages handler
            } else if message.topic == Topic::Zone.get() {
                // Fix for delayed behaviour in slow zones
                let prev_inside_slow_zone = self.inside_slow_zone.clone();

                self.inside_slow_zone = match payload["payload"]["value"].as_array() {
                    Some(x) => x.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
                    None => continue,
                };
                dbg!(&self.inside_slow_zone);

                // Fix for delayed behaviour in slow zones
                for vehicle in &prev_inside_slow_zone {
                    if !self.inside_slow_zone.contains(vehicle) {
                        client.publish(
                            &Topic::VehicleI(vehicle).get(),
                            &Payload::Speed(self.last_speed as i16, 500).get(),
                        );
                    }
                }

                for vehicle in &self.inside_slow_zone {
                    client.publish(
                        &Topic::VehicleI(vehicle).get(),
                        &Payload::Speed(200, 1000).get(),
                    );
                }

            // Any other message that will either get relayed or be overwritten
            } else {
                // Check if message.topic is correct (that is, it has a relayed topic in front)
                // Or continue loop and handle next message
                if !message.topic.contains(&Topic::Relay("").get()) {
                    dbg!("message topic doesn't have relay prefix");
                    continue;
                }
                // Extract topic and vehicle ID from message.topic
                let (_, topic) = message.topic.split_at(Topic::Relay("").get().len());
                let vehicle_id = topic.split('/').collect::<Vec<&str>>()[3].to_string();

                let payload_received =
                    String::from_utf8(message.payload.to_vec()).expect("should be valid utf8");

                let payload_sent = if payload["type"] == "speed" {
                    self.last_speed = payload["payload"]["velocity"]
                        .as_i64()
                        .expect("should have a valid speed value");

                    if self.emergency {
                        Payload::Speed(0, 2000).get()
                    } else if self.inside_slow_zone.contains(&vehicle_id) {
                        Payload::Speed(200, 1000).get()
                    } else {
                        payload_received
                    }
                } else {
                    payload_received
                };
                client.publish(topic, &payload_sent);
                //dbg!(payload_sent);
            }
        }
    }

    /// Run the client and return it's thread handle.
    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.loop_forever();
        })
    }
}
