use crate::{ClientWrapper, Payload, Topic};
use rumqttc::Publish;
use std::{io, sync::mpsc::Receiver, thread, time::Duration};

/// Sends Connect(true) to each vehicle.
pub fn connect_vehicles(client: &mut ClientWrapper, vehicle_list: &Vec<String>) {
    for vehicle in vehicle_list {
        client.publish(
            &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
            &Payload::Connect(true).get(),
        );
    }
}

/// Asks for discovered vehicle IDs, prints them, and terminates program.
pub fn discover_vehicles(
    client: &mut ClientWrapper,
    receiver: &Receiver<Publish>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("No vehicles specified. \nDiscovering vehicles...");
    client.subscribe(&Topic::HostS("vehicles").get());
    client.publish(&Topic::HostI.get(), &Payload::Discover(true).get());

    let received_payload = receiver.recv()?.payload;
    let payload: serde_json::Value = serde_json::from_slice(&received_payload)?;
    let available_vehicles = payload["value"]
        .as_array()
        .ok_or("None error")?
        .iter()
        .map(|v| v.as_str().expect("Should be valid UTF-8").to_string())
        .collect::<Vec<String>>();

    client.publish(&Topic::HostI.get(), &Payload::Discover(false).get());

    // Test if necessary (TODO)
    std::thread::sleep(Duration::from_millis(30));
    Ok(available_vehicles)
}

/// Blocks thread and publishes emergency messages on keypresses of enter
pub fn blocking_emergency_handler(client: &mut ClientWrapper) {
    let mut input = String::new();
    let mut state = false;

    println!("main: Press enter to toggle emergency state");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        state = !state;
        client.publish(&Topic::Emergency.get(), &Payload::Emergency(state).get());
    }
}

/// Sets up a handler to disconnect vehicles on CTRL+C
pub fn set_ctrlc_handler(client: &ClientWrapper, vehicle_list: &[String]) {
    let mut cloned_client = client.arc_clone();
    let cloned_vehicle_list = vehicle_list.to_owned();
    ctrlc::set_handler(move || {
        println!("Exiting...");

        for vehicle in &cloned_vehicle_list {
            cloned_client.publish(
                &Topic::Relay(&Topic::VehicleI(vehicle).get()).get(),
                &Payload::Connect(false).get(),
            );
        }

        thread::sleep(Duration::from_secs_f32(0.1));
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
