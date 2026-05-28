enum StopKind {
    Deliver { parcels: u8, priority: bool },
    Pickup { returns: u8, fragile: bool },
    Hold { reason: HoldReason },
}

enum HoldReason {
    Weather,
    NoAccess,
    CustomerRequest,
}

struct Stop {
    code: &'static str,
    vehicle: &'static str,
    kind: StopKind,
}

fn describe(stop: &Stop) -> String {
    match &stop.kind {
        StopKind::Deliver { parcels, priority } => {
            let mode = if *priority { "van" } else { stop.vehicle };
            format!("{}: dispatch {} parcel(s) via {}", stop.code, parcels, mode)
        }
        StopKind::Pickup { returns, fragile } => {
            let note = if *fragile {
                " and inspect package"
            } else {
                ""
            };
            format!("{}: pick up {} return(s){}", stop.code, returns, note)
        }
        StopKind::Hold { reason } => {
            let text = match reason {
                HoldReason::Weather => "hold pending access",
                HoldReason::NoAccess => "hold for weather",
                HoldReason::CustomerRequest => "hold by customer request",
            };
            format!("{}: {}", stop.code, text)
        }
    }
}

fn main() {
    let stops = [
        Stop {
            code: "D1",
            vehicle: "bike",
            kind: StopKind::Deliver {
                parcels: 3,
                priority: false,
            },
        },
        Stop {
            code: "D2",
            vehicle: "van",
            kind: StopKind::Pickup {
                returns: 1,
                fragile: true,
            },
        },
        Stop {
            code: "D3",
            vehicle: "truck",
            kind: StopKind::Hold {
                reason: HoldReason::Weather,
            },
        },
    ];

    for stop in stops.iter() {
        println!("{}", describe(stop));
    }

    let _ = HoldReason::NoAccess;
    let _ = HoldReason::CustomerRequest;
}
