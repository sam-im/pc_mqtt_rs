//! This lane module is part of the steering controller.
//!
//! This module lane contains the Lane struct and its implementation.
use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

/// Struct holding the offsets and a list of vehicles.
pub struct Lane {
    vehicles: Vec<String>,
    offsets: Vec<i16>,
}

impl Lane {
    /// Creates a new instance of Lane.
    pub fn new(offsets: &[i16], vehicles: &[String]) -> Self {
        Lane {
            offsets: offsets.to_owned(),
            vehicles: vehicles.to_owned(),
        }
    }

    /// Main logic of the lane client.
    ///
    /// Runs an infinite loop in a new thread, consuming the self and returning a handle to the thread.
    ///
    /// The loop publishes different offsets in lane message every 5 seconds for each vehicle in @vehicle_list.
    pub fn run(self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_lane");
        let _rx = connection.start_loop();

        thread::spawn(move || {
            let mut i = 0;
            loop {
                for vehicle in &self.vehicles {
                    client.publish(
                        &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                        &Payload::Lane(self.offsets[i], 200, 500).get(), //&Payload::Lane(0, 200, 500).get() // for testing
                    );
                }
                i = (i + 1) % self.offsets.len();
                thread::sleep(Duration::from_secs(5));
            }
        })
    }
}
