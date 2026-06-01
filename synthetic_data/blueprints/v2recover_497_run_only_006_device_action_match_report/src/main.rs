enum DeviceKind {
    Lamp,
    Fan,
    Door,
    Sensor,
}

enum Action {
    Power { on: bool },
    Speed(u8),
    Lock { locked: bool, jammed: bool },
    Fault(Option<&'static str>),
}

struct Event {
    name: &'static str,
    kind: DeviceKind,
    action: Action,
}

fn describe(event: &Event) -> String {
    let detail = match (&event.kind, &event.action) {
        (DeviceKind::Lamp, Action::Power { on }) => {
            if *on { "power(on)".to_string() } else { "power(off)".to_string() }
        }
        (DeviceKind::Fan, Action::Speed(level)) => format!("speed({})", level),
        (DeviceKind::Door, Action::Lock { locked, jammed }) => {
            match (*locked, *jammed) {
                (true, false) => "lock(engaged)".to_string(),
                (false, false) => "lock(released)".to_string(),
                (_, true) => "lock(blocked:battery_low)".to_string(),
            }
        }
        (DeviceKind::Sensor, Action::Fault(reason)) => {
            match reason {
                Some(msg) => format!("fault({})", msg),
                None => "fault(clear)".to_string(),
            }
        }
        _ => "invalid".to_string(),
    };

    format!("{} => {}", event.name, detail)
}

fn main() {
    let events = [
        Event {
            name: "lamp",
            kind: DeviceKind::Lamp,
            action: Action::Power { on: true },
        },
        Event {
            name: "fan",
            kind: DeviceKind::Fan,
            action: Action::Speed(3),
        },
        Event {
            name: "door",
            kind: DeviceKind::Door,
            action: Action::Lock {
                locked: true,
                jammed: true,
            },
        },
        Event {
            name: "sensor",
            kind: DeviceKind::Sensor,
            action: Action::Fault(Some("offline")),
        },
    ];

    for (i, event) in events.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{}", describe(event));
    }
}
