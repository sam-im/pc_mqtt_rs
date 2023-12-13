use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

pub struct Speed {
    velocity_list: Vec<i16>,
    vehicle_list: Vec<String>,
}

impl Speed {
    pub fn new(velocity_list: &[i16], vehicle_list: &[String]) -> Self {
        Speed {
            velocity_list: velocity_list.to_owned(),
            vehicle_list: vehicle_list.to_owned(),
        }
    }
    pub fn run(self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_speed");
        let _rx = connection.start_loop();

        thread::spawn(move || {
            let mut i = 0;
            loop {
                for vehicle in &self.vehicle_list {
                    client.publish(
                        &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                        &Payload::Speed(self.velocity_list[i], 1000).get(),
                    );
                }
                i = (i + 1) % self.velocity_list.len();
                thread::sleep(Duration::from_secs(3));
            }
        })
    }
}
