#![allow(dead_code)]
use serde_json::json;
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
