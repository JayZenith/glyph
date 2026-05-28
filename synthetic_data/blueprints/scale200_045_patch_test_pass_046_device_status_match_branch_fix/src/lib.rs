#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceState {
    Offline,
    Booting { progress: u8 },
    Online { healthy: bool, connections: u32 },
    Fault { code: u16, retryable: bool },
}

pub fn status_line(state: &DeviceState) -> String {
    match state {
        DeviceState::Offline => "offline".to_string(),
        DeviceState::Booting { progress } => format!("booting:{}%", progress.min(&100)),
        DeviceState::Online {
            healthy,
            connections,
        } => {
            if *healthy {
                format!("online:{} clients", connections)
            } else {
                "degraded".to_string()
            }
        }
        DeviceState::Fault { code, retryable } => {
            if *retryable {
                format!("fault {} (fatal)", code)
            } else {
                format!("fault {} (retry)", code)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{status_line, DeviceState};

    #[test]
    fn offline_and_booting_are_reported() {
        assert_eq!(status_line(&DeviceState::Offline), "offline");
        assert_eq!(
            status_line(&DeviceState::Booting { progress: 7 }),
            "booting:7%"
        );
        assert_eq!(
            status_line(&DeviceState::Booting { progress: 255 }),
            "booting:100%"
        );
    }

    #[test]
    fn online_variants_distinguish_health() {
        assert_eq!(
            status_line(&DeviceState::Online {
                healthy: true,
                connections: 3,
            }),
            "online:3 clients"
        );
        assert_eq!(
            status_line(&DeviceState::Online {
                healthy: false,
                connections: 9,
            }),
            "degraded:9 clients"
        );
    }

    #[test]
    fn fault_variants_label_retryability_correctly() {
        assert_eq!(
            status_line(&DeviceState::Fault {
                code: 42,
                retryable: true,
            }),
            "fault 42 (retry)"
        );
        assert_eq!(
            status_line(&DeviceState::Fault {
                code: 99,
                retryable: false,
            }),
            "fault 99 (fatal)"
        );
    }
}
