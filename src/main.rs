pub mod client;
pub mod library;

use crate::{
    client::{blink::*, lane::*, relay::*, speed::*, track::*},
    library::{mqtt::*, payload::*, topic::*},
};
use rumqttc::Publish;
use std::{io, sync::mpsc::Receiver, thread, time::Duration};

fn main() {
    let vehicle_list: Vec<String> = vec![
        String::from("f2e85f2f5770"),
        String::from("d98ebab7c206"),
        String::from("c60ee9d05225"),
        String::from("c40caf091413"),
        String::from("d11d2fea5c74"),
    ];
    //let vehicle_list: Vec<String> = vec![];

    let slow_tracks = vec![20, 4, 21];

    // Shared client for functions defined in main.rs
    let (mut client, connection) = Mqtt::new("groupg_main");
    let rx = connection.start_loop();

    if vehicle_list.len() == 0 {
        discover_vehicles(&mut client, &rx);
    }

    // Starting relay first to avoid lost connect messages (TODO)
    let _relay = Relay::new(&vehicle_list).run();
    thread::sleep(Duration::from_millis(30)); // Hack for lost connect messages (TODO)

    // Connect to vehicles and start the clients
    connect_vehicles(&mut client, &vehicle_list);
    let _blink = Blink::new(&vehicle_list).run();
    let _speed = Speed::new(&vehicle_list).run();
    let _lane = Lane::new(&vehicle_list).run();
    let _track = Track::new(&vehicle_list, &slow_tracks).run();

    set_ctrlc_handler(&client, &vehicle_list);
    blocking_emergency_handler(&mut client);
}

/// Sends Connect(true) to each vehicle.
fn connect_vehicles(client: &mut ClientWrapper, vehicle_list: &Vec<String>) {
    for vehicle in vehicle_list {
        client.publish(
            &Topic::Relay(&Topic::VehicleI(&vehicle).get()).get(),
            &Payload::Connect(true).get(),
        );
    }
}

/// Asks for discovered vehicle IDs, prints them, and terminates program.
fn discover_vehicles(client: &mut ClientWrapper, receiver: &Receiver<Publish>) {
    println!("No vehicles specified.");

    client.subscribe(&Topic::HostS("vehicles").get());
    client.publish(&Topic::HostI.get(), &Payload::Discover(true).get());

    let received = receiver.recv().expect("should receive Ok(val)");
    let payload: serde_json::Value =
        serde_json::from_slice(&received.payload).expect("should be valid json");
    let available_vehicles = payload["value"].as_array().expect("should be an array");

    println!("Available vehicles:");
    for vehicle in available_vehicles {
        println!("  {}", vehicle);
    }

    client.publish(&Topic::HostI.get(), &Payload::Discover(false).get());

    thread::sleep(Duration::from_millis(30)); // Check if this is needed
    std::process::exit(0);
}

/// Blocks thread and publishes emergency messages on keypress of enter
fn blocking_emergency_handler(client: &mut ClientWrapper) {
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
fn set_ctrlc_handler(client: &ClientWrapper, vehicle_list: &Vec<String>) {
    let mut cloned_client = client.arc_clone();
    let cloned_vehicle_list = vehicle_list.clone();
    ctrlc::set_handler(move || {
        println!("Exiting...");

        for vehicle in &cloned_vehicle_list {
            cloned_client.publish(
                &Topic::Relay(&Topic::VehicleI(&vehicle).get()).get(),
                &Payload::Connect(false).get(),
            );
        }

        thread::sleep(Duration::from_secs_f32(0.1));
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
