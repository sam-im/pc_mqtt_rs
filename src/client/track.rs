//! Track module which also includes the personal addition.
//! 
//! The fields of the struct are:
//! * vehicle_list: A list of vehicle IDs that this client should connect to.
//! * slow_tracks: A list of track IDs that the vehicles should go slower (velocity 200).
//! * slow_vehicles: A list of vehicle IDs that are currently on the said slow tracks.
//! 
//! This client subscribes to the event topic of each vehicle, receiving track ID messages.
//! The client will only publish messages if there are any changes to the slow_vehicles list.

use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::thread;

pub struct Track {
    vehicle_list: Vec<String>,
    slow_tracks: Vec<u64>,
    slow_vehicles: Vec<String>,
}

impl Track {
    pub fn new(vehicle_list: &[String], slow_tracks: &[u64]) -> Self {
        Track {
            vehicle_list: vehicle_list.to_owned(),
            slow_tracks: slow_tracks.to_owned(),
            slow_vehicles: Vec::new(),
        }
    }

    pub fn run(mut self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_track");

        for vehicle in &self.vehicle_list {
            client.subscribe(&Topic::TrackE(vehicle).get());
        }

        thread::spawn(move || {
            for message in connection.start_loop() {
                let vehicle_id = message.topic.split('/').collect::<Vec<&str>>()[3].to_string();

                let track_id = {
                    let payload: serde_json::Value = match serde_json::from_slice(&message.payload)
                    {
                        Ok(payload) => payload,
                        Err(e) => {
                            dbg!("{}", e);
                            continue;
                        }
                    };
                    match payload["trackId"].as_u64() {
                        Some(track_id) => track_id,
                        None => {
                            dbg!("payload[\"trackId\"] returned None");
                            continue;
                        }
                    }
                };
                dbg!(&track_id);

                // Update and publish slow_vehicles list only if necessary
                if self.slow_tracks.contains(&track_id) && !self.slow_vehicles.contains(&vehicle_id)
                {
                    self.slow_vehicles.push(vehicle_id.clone());

                    // Publish current list
                    client.publish(
                        &Topic::Zone.get(),
                        &Payload::Zone200(&self.slow_vehicles).get(),
                    );
                } else if !self.slow_tracks.contains(&track_id)
                    && self.slow_vehicles.contains(&vehicle_id)
                {
                    self.slow_vehicles.sort();
                    self.slow_vehicles
                        .binary_search(&vehicle_id)
                        .ok()
                        .map(|i| self.slow_vehicles.remove(i));

                    // Publish current list
                    client.publish(
                        &Topic::Zone.get(),
                        &Payload::Zone200(&self.slow_vehicles).get(),
                    );
                }
            }
        })
    }
}
