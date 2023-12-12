use crate::library::{mqtt::Mqtt, payload::Payload, topic::Topic};
use std::{thread, time::Duration};

pub struct Blink {
    state: bool,
    vehicle_list: Vec<String>,
}

impl Blink {
    pub fn new(vehicle_list: &Vec<String>) -> Self {
        Blink {
            state: false,
            vehicle_list: vehicle_list.clone(),
        }
    }

    pub fn run(mut self) -> thread::JoinHandle<()> {
        let (mut client, connection) = Mqtt::new("groupg_blink");
        let _rx = connection.start_loop();

        thread::spawn(move || loop {
            self.state = !self.state;

            for vehicle in &self.vehicle_list {
                client.publish(
                    &Topic::Relay(&Topic::VehicleI(&vehicle).get()).get(),
                    &Payload::Lights(self.state, self.state).get(),
                );
            }
            thread::sleep(Duration::from_secs(1));
        })
    }
}
