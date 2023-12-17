//! This module contains payloads/messages used in the project, making it both easier to use and change them later on.

#![allow(dead_code)]
use serde_json::json;

/// An enum that holds most of the payloads/messagess used in the project.
pub enum Payload<'a> {
    Speed(i16, u16),
    Connect(bool),
    Discover(bool),
    Lane(i16, u16, u16),
    Lights(bool, bool),
    Emergency(bool),
    Zone200(&'a Vec<String>),
}

impl Payload<'_> {
    /// Formats hardcoded payload strings with the values inside the enum instances, if any.
    /// # Example
    /// ```
    /// use pc_mqtt_rs::Payload;
    ///
    /// assert_eq!(Payload::Speed(200, 1000).get(), String::from(r#"{"type":"speed","payload":{"velocity": 200,"acceleration": 1000}}"#));
    /// ```
    pub fn get(&self) -> String {
        match self {
            Payload::Speed(vel, acc) => {
                format!(
                    r#"{{"type":"speed","payload":{{"velocity": {},"acceleration": {}}}}}"#,
                    vel, acc
                )
            }
            Payload::Connect(value) => {
                format!(r#"{{"type":"connect","payload":{{"value":{}}}}}"#, value)
            }
            Payload::Discover(value) => {
                format!(r#"{{"type":"discover","payload":{{"value":{}}}}}"#, value)
            }
            Payload::Lane(offset, velocity, acceleration) => {
                format!(
                    r#"{{"type":"lane","payload":{{"offset":{},"velocity":{},"acceleration":{}}}}}"#,
                    offset, velocity, acceleration
                )
            }
            Payload::Lights(front, back) => {
                format!(
                    r#"{{"type":"lights","payload":{{"front":"{}","back":"{}"}}}}"#,
                    if *front { "on" } else { "off" },
                    if *back { "on" } else { "off" }
                )
            }
            Payload::Emergency(value) => {
                format!(r#"{{"type":"emergency","payload":{{"value":{}}}}}"#, value)
            }
            Payload::Zone200(value) => serde_json::to_string(&json!({
                "type": "zone200",
                "payload": {
                    "value": value
                }
            }))
            .expect("should be Ok(String)"),
        }
    }
}
