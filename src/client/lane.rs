use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

/// Holds a list of offsets and a list of vehicles IDs.
/// 
/// The list offset is 
pub struct Lane {
    offset: Box<[i16]>,
    vehicle_list: Vec<String>,
}

impl Lane {
    /// Returns a new instance of Lane.
    pub fn new(offset: &[i16], vehicle_list: &[String]) -> Self {
        Lane {
            offset: offset.to_owned().into_boxed_slice(),
            vehicle_list: vehicle_list.to_owned(),
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
                for vehicle in &self.vehicle_list {
                    client.publish(
                        &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                        &Payload::Lane(self.offset[i], 200, 500).get(), //&Payload::Lane(0, 200, 500).get() // for testing
                    );
                }
                i = (i + 1) % self.offset.len();
                thread::sleep(Duration::from_secs(5));
            }
        })
    }
}
