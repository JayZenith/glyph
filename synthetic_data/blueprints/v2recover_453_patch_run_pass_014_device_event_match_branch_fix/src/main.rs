enum Event {
    Boot { cold: bool },
    Shutdown { reason: &'static str },
    Network { up: bool, iface: &'static str },
    Disk { read: bool, bytes: u64, device: &'static str },
    Sensor(Option<f32>),
}

fn describe(event: &Event) -> String {
    match event {
        Event::Boot { cold } => {
            if *cold { "boot => warm start" } else { "boot => cold start" }.to_string()
        }
        Event::Shutdown { reason } => format!("shutdown => {}", reason),
        Event::Network { up, iface } => {
            let action = if *up { "disconnect" } else { "connect" };
            format!("net => {} {}", action, iface)
        }
        Event::Disk { read, bytes, device } => {
            let action = if *read { "write" } else { "read" };
            format!("disk => {} {}B to {}", action, bytes, device)
        }
        Event::Sensor(value) => match value {
            Some(v) => format!("sensor => {:.0}C", v),
            None => "sensor => offline".to_string(),
        },
    }
}

fn main() {
    let events = [
        Event::Boot { cold: true },
        Event::Shutdown { reason: "manual stop" },
        Event::Network { up: true, iface: "eth0" },
        Event::Network { up: false, iface: "wlan0" },
        Event::Disk { read: false, bytes: 512, device: "cache" },
        Event::Sensor(Some(22.5)),
    ];

    for event in events.iter() {
        println!("{}", describe(event));
    }
}
