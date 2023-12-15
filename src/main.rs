use pc_mqtt_rs::*;
use std::{thread, time::Duration};

fn main() {
    let vehicle_list: Vec<String> = vec![
        String::from("f2e85f2f5770"),
        //String::from("d98ebab7c206"),
        //String::from("c60ee9d05225"),
        //String::from("c40caf091413"),
        //String::from("d11d2fea5c74"),
    ];

    // Shared MQTT client for helper function such as discover_vehicles, connect_vehicles, etc.
    let (mut client, connection) = Mqtt::new("groupg_main");
    // Channel receiver to receive messages from a connection loop. This specific one is only used by the discover_vehicles function.
    let rx = connection.start_loop();

    // Discover and print vehicles IDs if none are specified
    if vehicle_list.is_empty() {
        discover_vehicles(&mut client, &rx);
    }

    // Start relay first to avoid lost connect messages
    let _relay = Relay::new(&vehicle_list).run();
    thread::sleep(Duration::from_millis(30)); // Hack for lost connect messages (TODO)

    // Connect to vehicles and start the clients
    connect_vehicles(&mut client, &vehicle_list);
    let _blink = Blink::new(&vehicle_list).run();
    let _speed = Speed::new(&[300, 400, 500], &vehicle_list).run();
    let _lane = Lane::new(&[0, 10, 0, -10], &vehicle_list).run();
    let _track = Track::new(&vehicle_list, &[20, 4, 21]).run();

    // CTRL+C handler to disconnect vehicles on exit
    set_ctrlc_handler(&client, &vehicle_list);

    // Block thread and publish emergency messages on keypresses of enter
    blocking_emergency_handler(&mut client);
}

