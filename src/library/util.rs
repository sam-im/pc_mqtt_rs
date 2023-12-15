use crate::{ClientWrapper, Payload, Topic};
use std::{io, thread, time::Duration, sync::mpsc::Receiver};
use rumqttc::Publish;

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
pub fn discover_vehicles(client: &mut ClientWrapper, receiver: &Receiver<Publish>) {
    client.subscribe(&Topic::HostS("vehicles").get());
    client.publish(&Topic::HostI.get(), &Payload::Discover(true).get());

    let received = receiver.recv().expect("should receive Ok(val)");
    let payload: serde_json::Value =
        serde_json::from_slice(&received.payload).expect("should be valid json");
    let available_vehicles = payload["value"].as_array().expect("should be an array");
    
    println!("No vehicles specified \n Available vehicles:");
    for vehicle in available_vehicles {
        println!("  {}", vehicle);
    }

    client.publish(&Topic::HostI.get(), &Payload::Discover(false).get());

    thread::sleep(Duration::from_millis(30)); // Check if this is needed
    std::process::exit(0);
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
