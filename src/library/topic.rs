//! This module contains the topics, making it both easier to use and change them later on.

#![allow(dead_code)]

/// An enum that holds almost all the topics used in the project.
pub enum Topic<'a> {
    HostI,
    HostS(&'a str),
    VehicleI(&'a str),
    VehicleS(&'a str),
    Relay(&'a str),
    TrackE(&'a str),
    SpeedE(&'a str),
    Emergency,
    Zone,
}

impl Topic<'_> {
    /// Formats hardcoded topic strings with the values inside the enum instances, if any.
    /// # Example
    /// ```
    /// use pc_mqtt_rs::Topic;
    ///
    /// assert_eq!(Topic::VehicleI("test").get(), String::from("Anki/Vehicles/U/test/I"));
    /// ```
    pub fn get(self) -> String {
        match self {
            Topic::HostI => String::from("Anki/Hosts/U/hyperdrive/I"),
            Topic::HostS(val) => format!(r#"Anki/Hosts/U/hyperdrive/S/{}"#, val),
            Topic::VehicleI(val) => format!(r#"Anki/Vehicles/U/{}/I"#, val),
            Topic::VehicleS(val) => format!(r#"Anki/Vehicles/U/{}/S"#, val),
            Topic::Relay(val) => format!(r#"GroupG/Relay/{}"#, val),
            Topic::TrackE(val) => format!(r#"Anki/Vehicles/U/{}/E/track"#, val),
            Topic::SpeedE(val) => format!(r#"Anki/Vehicles/U/{}/E/speed"#, val),
            Topic::Emergency => String::from("GroupG/Emergency/I"),
            Topic::Zone => String::from(r#"GroupG/Zone/I"#),
        }
    }
}
