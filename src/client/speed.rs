//! This speed module is part of the steering controller.
//!
//! It contains the Speed struct and its implementation.
use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

/// Struct holding lists of velocities and vehicles.
pub struct Speed {
    velocity_list: Vec<i16>,
    vehicle_list: Vec<String>,
}

impl Speed {
    /// Creates a new instance of Speed.
    pub fn new(velocity_list: &[i16], vehicle_list: &[String]) -> Self {
        Speed {
            velocity_list: velocity_list.to_owned(),
            vehicle_list: vehicle_list.to_owned(),
        }
    }
    /// Main logic of the speed client.
    ///
    /// Runs an infinite loop in a new thread, consuming the self and returning a handle to the thread.
    ///
    /// The loop publishes different velocities in speed message every 3 seconds for each vehicle in vehicle_list.
    pub fn run(self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_speed");
        let _rx = connection.start_loop();

        thread::spawn(move || {
            let mut i = 0;
            loop {
                for vehicle in &self.vehicle_list {
                    client.publish(
                        &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                        &Payload::Speed(self.velocity_list[i], 500).get(),
                    );
                }
                i = (i + 1) % self.velocity_list.len();
                thread::sleep(Duration::from_secs(3));
            }
        })
    }
}
