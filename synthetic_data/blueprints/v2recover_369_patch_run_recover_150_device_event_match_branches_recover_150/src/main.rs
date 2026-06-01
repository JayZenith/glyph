enum Event {
    Sensor(SensorEvent),
    Motor(MotorEvent),
    Link(LinkEvent),
    Controller(ControllerEvent),
}

enum SensorEvent {
    Calibrated { id: u8, stable: bool },
    Reading { id: u8, celsius: f32 },
}

enum MotorEvent {
    Started { name: &'static str, rpm: u16 },
    Stopped { name: &'static str },
}

enum LinkEvent {
    Connected { name: &'static str },
    Disconnected { name: &'static str, timeout: bool },
}

enum ControllerEvent {
    Warning { unit: &'static str, code: &'static str },
    Stopped { unit: &'static str, by_operator: bool },
}

fn describe(event: &Event) -> String {
    match event {
        Event::Sensor(sensor) => match sensor {
            SensorEvent::Calibrated { id, stable } => {
                if *stable {
                    format!("Sensor #{}: calibrated", id)
                } else {
                    format!("Sensor #{}: calibrated (stable)", id)
                }
            }
            SensorEvent::Reading { id, celsius } => format!("Sensor {}: reading {:.1} C", id, celsius),
        },
        Event::Motor(motor) => match motor {
            MotorEvent::Started { name, rpm } => format!("Motor {}: started at {} rpm", name, rpm),
            MotorEvent::Stopped { name } => format!("Motor {}: stopped", name),
        },
        Event::Link(link) => match link {
            LinkEvent::Connected { name } => format!("Link {}: connected", name),
            LinkEvent::Disconnected { name, timeout } => {
                if *timeout {
                    format!("Link {}: disconnected", name)
                } else {
                    format!("Link {}: disconnected (timeout)", name)
                }
            }
        },
        Event::Controller(ctrl) => match ctrl {
            ControllerEvent::Warning { unit, code } => format!("Controller {}: stopped {}", unit, code),
            ControllerEvent::Stopped { unit, by_operator } => {
                if *by_operator {
                    format!("Controller {}: stopped automatically", unit)
                } else {
                    format!("Controller {}: stopped by operator", unit)
                }
            }
        },
    }
}

fn main() {
    let events = [
        Event::Sensor(SensorEvent::Calibrated { id: 4, stable: true }),
        Event::Sensor(SensorEvent::Reading {
            id: 4,
            celsius: 27.2,
        }),
        Event::Motor(MotorEvent::Started {
            name: "left",
            rpm: 900,
        }),
        Event::Motor(MotorEvent::Stopped { name: "left" }),
        Event::Link(LinkEvent::Connected { name: "uplink" }),
        Event::Link(LinkEvent::Disconnected {
            name: "uplink",
            timeout: true,
        }),
        Event::Controller(ControllerEvent::Warning {
            unit: "core",
            code: "low battery",
        }),
        Event::Controller(ControllerEvent::Stopped {
            unit: "core",
            by_operator: true,
        }),
    ];

    for event in events.iter() {
        println!("{}", describe(event));
    }
}
