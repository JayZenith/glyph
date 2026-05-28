enum Route {
    Air { pilot: &'static str, priority: bool },
    Ground { dock: &'static str, express: bool },
    Sea { harbor: &'static str, refrigerated: bool },
    Cancelled,
}

struct Shipment {
    id: &'static str,
    route: Route,
}

fn action(route: &Route) -> (&'static str, &'static str, &'static str) {
    match route {
        Route::Air { pilot, priority } => {
            let stage = if *priority { "queue" } else { "dispatch" };
            (stage, "air priority", pilot)
        }
        Route::Ground { dock, express } => {
            let label = if *express { "ground standard" } else { "ground express" };
            ("queue", label, dock)
        }
        Route::Sea { harbor, refrigerated } => {
            let stage = if *refrigerated { "deliver" } else { "queue" };
            let label = if *refrigerated { "sea chilled" } else { "sea freight" };
            (stage, label, harbor)
        }
        Route::Cancelled => ("reject", "canceled", "none"),
    }
}

fn main() {
    let shipments = [
        Shipment {
            id: "A1",
            route: Route::Air {
                pilot: "pilot",
                priority: true,
            },
        },
        Shipment {
            id: "B2",
            route: Route::Ground {
                dock: "dock",
                express: false,
            },
        },
        Shipment {
            id: "C3",
            route: Route::Sea {
                harbor: "harbor",
                refrigerated: false,
            },
        },
        Shipment {
            id: "D4",
            route: Route::Cancelled,
        },
    ];

    for shipment in shipments {
        let (verb, label, owner) = action(&shipment.route);
        println!("{} -> {} {} via {}", shipment.id, verb, label, owner);
    }
}
