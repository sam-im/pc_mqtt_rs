//! Track module which also includes the personal addition.
//!
//! The fields of the struct are:
//! * vehicle_list: A list of vehicle IDs that this client should connect to.
//! * slow_tracks: A list of track IDs that the vehicles should go slower (velocity 200).
//! * slow_vehicles: A list of vehicle IDs that are currently on the said slow tracks.
//!
//! This client subscribes to the event topic of each vehicle, receiving track ID and wheel distance messages.
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

    /// Main logic of the track client.
    ///
    /// Runs an infinite loop in a new thread, consuming the self and returning a handle to the thread.
    ///
    /// The loop subscribes to the event topics "track" and "wheelDistance" of each vehicle in vehicle_list.
    ///
    /// Whenever a new message is received, the data is extracted and saved to the corresponding variables.
    ///
    /// To control whether a vehicle is turning, the difference between the left and right wheel distance is calculated. If the difference is greater than 4, the vehicle is turning.
    ///
    /// A list of slow vehicles is maintained. If a vehicle is on a slow track and is not in the list, it is added to the list. If a vehicle is not on a slow track and is in the list, it is removed from the list. This list is published on update to the zone topic.
    pub fn run(mut self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_track");

        for vehicle in &self.vehicle_list {
            client.subscribe(&Topic::VehicleE(vehicle, "track").get());
            client.subscribe(&Topic::VehicleE(vehicle, "wheelDistance").get());
        }

        thread::spawn(move || {
            let mut track_id: u64 = 0;
            let mut prev_track_id: u64 = 0;
            let mut is_turning: bool = false;

            for message in connection.start_loop() {
                let vehicle_id = message.topic.split('/').collect::<Vec<&str>>()[3].to_string();
                let payload: serde_json::Value = match serde_json::from_slice(&message.payload) {
                    Ok(payload) => payload,
                    Err(e) => {
                        dbg!("{}", e);
                        continue;
                    }
                };

                if message.topic.contains("track") {
                    prev_track_id = track_id.clone();
                    track_id = {
                        match payload["trackId"].as_u64() {
                            Some(track_id) => track_id,
                            None => {
                                dbg!("payload[\"trackId\"] returned None");
                                continue;
                            }
                        }
                    };
                } else if message.topic.contains("wheelDistance") {
                    let left = {
                        match payload["left"].as_i64() {
                            Some(left) => left,
                            None => {
                                dbg!("left returned None");
                                continue;
                            }
                        }
                    };
                    let right = {
                        match payload["right"].as_i64() {
                            Some(right) => right,
                            None => {
                                dbg!("right returned None");
                                continue;
                            }
                        }
                    };
                    is_turning = if (left - right).abs() > 4 {
                        true
                    } else {
                        false
                    };
                }
                if track_id != prev_track_id {
                    println!("track: {}, is_turning: {}", track_id, is_turning);
                }

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
