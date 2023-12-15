//! This module contains the MQTT client and connection wrappers.
//!
//! To create a new client and connection pair use the new function.
//! To maintain connection and receive incoming publish event notifications use the start_loop function.

#![allow(dead_code)]

use rumqttc::{Client, Connection, Event, Incoming, MqttOptions, Publish, QoS};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Mqtt {}

impl Mqtt {
    /// Creates a new MQTT Client/Connection pair.
    /// # Example
    /// ```
    /// use pc_mqtt_rs::Mqtt;
    /// 
    /// let (mut client, connection) = Mqtt::new("doc_test");
    /// let rx = connection.start_loop();
    /// 
    /// client.subscribe("test/topic");
    /// client.publish("test/topic", "test-payload");
    /// 
    /// let received = rx.recv().unwrap().payload;
    /// assert_eq!(received, "test-payload");
    /// ```
    pub fn new(client_id: &str) -> (ClientWrapper, ConnectionWrapper) {
        let (client, connection) = Mqtt::init_client(client_id);
        let client = Arc::new(Mutex::new(client));
        (ClientWrapper { client }, ConnectionWrapper { connection })
    }

    fn set_options(client_id: &str) -> MqttOptions {
        //let mut options = MqttOptions::new(client_id, "192.168.4.1", 1883);
        let mut options = MqttOptions::new(client_id, "147.87.116.34", 1883); // For the non-PI broker
        options
            .set_credentials("cedalo", "gCgZnxzl3liLFPCe5Vom2t5Ha") // For the non-PI broker
            .set_transport(rumqttc::Transport::Tcp)
            .set_keep_alive(Duration::from_secs(60));

        options
    }

    fn init_client(client_id: &str) -> (Client, Connection) {
        let (client, connection) = Client::new(Mqtt::set_options(client_id), 10);
        (client, connection)
    }
}

/// Rumqttc client wrapper.
/// It wraps the client in an Arc<Mutex<>> to allow sharing it between threads safely.
pub struct ClientWrapper {
    client: Arc<Mutex<Client>>,
}

impl ClientWrapper {
    pub fn publish(&mut self, topic: &str, payload: &str) {
        self.client
            .lock()
            .unwrap()
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .unwrap();
    }

    pub fn subscribe(&mut self, topic: &str) {
        self.client
            .lock()
            .unwrap()
            .subscribe(topic, QoS::AtLeastOnce)
            .unwrap();
    }

    pub fn unsubscribe(&mut self, topic: &str) {
        self.client.lock().unwrap().unsubscribe(topic).unwrap();
    }

    pub fn arc_clone(&self) -> Self {
        ClientWrapper {
            client: self.client.clone(),
        }
    }
}

pub struct ConnectionWrapper {
    connection: Connection,
}

impl ConnectionWrapper {
    /// Iterates over Connection and send incoming publish event notifications over returned receiver.
    pub fn start_loop(mut self) -> mpsc::Receiver<Publish> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for notification in self.connection.iter() {
                // send over only incoming publish event notifications
                if let Ok(Event::Incoming(Incoming::Publish(notification))) = notification {
                    match tx.send(notification) {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!("send attempt failed: {}", e);
                            continue;
                        }
                    }
                }
            }
        });
        rx
    }
}
