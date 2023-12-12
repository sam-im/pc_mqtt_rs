use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

pub struct Speed {
    velocity: i16,
    base_velocity: i16,
    vehicle_list: Vec<String>,
}

impl Speed {
    pub fn new(vehicle_list: &[String]) -> Self {
        Speed {
            velocity: 0,
            base_velocity: 200,
            vehicle_list: vehicle_list.to_owned(),
        }
    }
    pub fn run(mut self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_speed");
        let _rx = connection.start_loop();

        thread::spawn(move || {
            loop {
                self.velocity = (self.velocity + 100) % 400;
                let speed = self.base_velocity + self.velocity;

                for vehicle in &self.vehicle_list {
                    client.publish(
                        &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                        &Payload::Speed(speed, 1000).get(), //&Payload::Speed(500, 1000).get() // for testing
                    );
                }
                thread::sleep(Duration::from_secs(3));
            }
        })
    }
}
