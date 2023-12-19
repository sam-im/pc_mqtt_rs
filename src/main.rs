use pc_mqtt_rs::*;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CONFIG START HERE
    let vehicle_list: Vec<String> = vec![
        //String::from("f4c22c6c0382"),
        //String::from("cec233dec1cb"),
        String::from("f2e85f2f5770"),
        //String::from("d98ebab7c206"),
        //String::from("c60ee9d05225"),
        //String::from("c40caf091413"),
        //String::from("d11d2fea5c74"),
    ];

    // For steering and track demonstration
    //let speed_list = vec![300, 400, 500];
    //let lane_list = vec![-30, 0];
    //let slow_tracks = vec![];

    // For personal addition demonstration
    let speed_list = vec![500];
    let lane_list = vec![0];
    let slow_tracks = vec![20, 4, 21];
    // CONFIG END HERE

    // Shared MQTT client for helper function such as discover_vehicles, connect_vehicles, etc.
    let (mut client, connection) = Mqtt::new("groupg_main");
    // Channel receiver to receive messages from a connection loop. This specific one is only used by the discover_vehicles function.
    let rx = connection.start_loop();

    // Discover and print vehicles IDs if none are specified
    if vehicle_list.is_empty() {
        for vehicle in discover_vehicles(&mut client, &rx)? {
            println!("  {}", vehicle);
        }
        std::process::exit(0);
    }

    // Start relay first to avoid lost connect messages
    let _relay = Relay::new(&vehicle_list).run();
    thread::sleep(Duration::from_millis(30)); // Hack for lost connect messages (TODO)

    connect_vehicles(&mut client, &vehicle_list);
    let _blink = Blink::new(&vehicle_list).run();
    let _speed = Speed::new(&speed_list, &vehicle_list).run();
    let _lane = Lane::new(&lane_list, &vehicle_list).run();
    let _track = Track::new(&vehicle_list, &slow_tracks).run();

    // CTRL+C handler to disconnect vehicles on exit
    set_ctrlc_handler(&client, &vehicle_list);

    // Block thread and publish emergency messages on keypresses of enter
    blocking_emergency_handler(&mut client);

    Ok(())
}
