use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

pub struct Lane {
    offset: [i16; 4],
    vehicle_list: Vec<String>,
}

impl Lane {
    pub fn new(vehicle_list: &[String]) -> Self {
        Lane {
            offset: [0, 10, 0, -10],
            vehicle_list: vehicle_list.to_owned(),
        }
    }

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
                i = (i + 1) % 4;
                thread::sleep(Duration::from_secs(5));
            }
        })
    }
}
