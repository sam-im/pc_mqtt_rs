//! The blink module is the simplest of the three steering controllers. When run it will toggle the lights of all vehicles in the vehicle list every second. The current state of the lights are stored in the state field.
use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

/// Holds the current state and a list of vehicles.
pub struct Blink {
    /// Current state of the lights.
    state: bool,
    /// A list of vehicles IDs.
    vehicles: Vec<String>,
}

impl Blink {
    /// Returns a new instance of Blink.
    pub fn new(vehicle_list: &[String]) -> Self {
        Blink {
            state: false,
            vehicles: vehicle_list.to_owned(),
        }
    }

    /// Runs the client in a separate thread, consuming it.
    ///
    /// Returns a handle to the thread.
    pub fn run(mut self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_blink");
        let _rx = connection.start_loop();

        thread::spawn(move || loop {
            self.state = !self.state;

            for vehicle in &self.vehicles {
                client.publish(
                    &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                    &Payload::Lights(self.state, self.state).get(),
                );
            }
            thread::sleep(Duration::from_secs(1));
        })
    }
}
