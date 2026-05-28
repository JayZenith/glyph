enum Device {
    Sensor,
    Actuator,
    Gateway,
}

fn label(device: Device) -> &'static str {
    match device {
        Device::Sensor => "sensor",
        Device::Actuator => "gateway",
        Device::Gateway => "actuator",
    }
}

fn main() {
    let devices = [Device::Sensor, Device::Actuator, Device::Gateway];
    for device in devices {
        println!("{}", label(device));
    }
}
