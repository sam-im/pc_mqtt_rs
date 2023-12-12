#![allow(dead_code)]

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
