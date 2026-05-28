enum DeviceEvent {
    Heater { temp: i32, occupied: bool },
    Light { level: u8, motion: bool },
    Door { open: bool, locked: bool, label: &'static str },
}

fn decide(event: &DeviceEvent) -> String {
    match event {
        DeviceEvent::Heater { temp, occupied } => {
            if *temp < 18 {
                format!("heater:warm({})", 18 - temp)
            } else if *occupied {
                "heater:idle".to_string()
            } else {
                "heater:eco".to_string()
            }
        }
        DeviceEvent::Light { level, motion } => {
            if *motion && *level == 0 {
                "light:on".to_string()
            } else if *level > 0 {
                format!("light:dim({})", level)
            } else {
                "light:off".to_string()
            }
        }
        DeviceEvent::Door { open, locked, label } => {
            if *open {
                format!("door:close({})", label)
            } else if *locked {
                format!("door:unlock({})", label)
            } else {
                format!("door:watch({})", label)
            }
        }
    }
}

fn main() {
    let events = [
        DeviceEvent::Heater { temp: 15, occupied: true },
        DeviceEvent::Light { level: 0, motion: true },
        DeviceEvent::Door { open: false, locked: true, label: "front" },
        DeviceEvent::Light { level: 40, motion: false },
        DeviceEvent::Heater { temp: 21, occupied: true },
    ];

    for event in events.iter() {
        println!("{}", decide(event));
    }
}
