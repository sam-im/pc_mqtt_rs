use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use serde_json;
use std::thread::{self};

pub struct Relay {
    vehicle_list: Vec<String>,
    emergency: bool,
    inside_slow_zone: Vec<String>,
}

impl Relay {
    pub fn new(vehicle_list: &[String]) -> Relay {
        Relay {
            vehicle_list: vehicle_list.to_owned(),
            emergency: false,
            inside_slow_zone: Vec::new(),
        }
    }

    /// For each message received from
    fn loop_forever(mut self) {
        let (mut client, connection) = Mqtt::new("group-g_relay");
        client.subscribe(&Topic::Relay("#").get());
        client.subscribe(&Topic::Emergency.get());
        client.subscribe(&Topic::Zone.get());

        // Fix for delayed behaviour in slow zones
        let mut last_speed: i16 = 200;

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
                            &Payload::Speed(last_speed, 500).get(),
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
                // Check if message.topic is corrent (that is it has a relayed topic in front)
                // Or continue loop and handle next message
                if !message.topic.contains(&Topic::Relay("").get()) {
                    dbg!("message topic doesn't have relay prefix");
                    continue;
                }
                // Extract topic and vehicle ID from message.topic
                let (_, topic) = message.topic.split_at(Topic::Relay("").get().len());
                let vehicle_id = topic.split('/').collect::<Vec<&str>>()[3].to_string();

                let original_payload =
                    String::from_utf8(message.payload.to_vec()).expect("should be valid utf8");

                let new_payload = if payload["type"] == "speed" {
                    last_speed = payload["payload"]["velocity"]
                        .as_i64()
                        .expect("should return a valid speed payload")
                        as i16;
                    if self.emergency {
                        Payload::Speed(0, 2000).get()
                    } else if self.inside_slow_zone.contains(&vehicle_id) {
                        Payload::Speed(200, 1000).get()
                    } else {
                        original_payload
                    }
                } else {
                    original_payload
                };
                client.publish(topic, &new_payload);
                //dbg!(new_payload);
            }
        }
    }

    /// Run the client (usually indefinitely) and return it's thread handle
    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.loop_forever();
        })
    }
}
