enum Event {
    Created(String),
    Updated { name: String, version: u8 },
    Deleted { name: String, hard: bool },
    Archived(String),
}

fn describe(event: &Event) -> String {
    match event {
        Event::Created(name) => format!("created:{name}"),
        Event::Updated { name, version } => format!("updated:{name}@{version}"),
        Event::Deleted { name, hard } => {
            if *hard {
                format!("deleted:{name} (soft)")
            } else {
                format!("deleted:{name} (hard)")
            }
        }
        Event::Archived(name) => format!("deleted:{name} (soft)"),
    }
}

fn main() {
    let events = [
        Event::Created("alpha".to_string()),
        Event::Updated {
            name: "beta".to_string(),
            version: 3,
        },
        Event::Deleted {
            name: "gamma".to_string(),
            hard: false,
        },
        Event::Deleted {
            name: "delta".to_string(),
            hard: true,
        },
        Event::Archived("epsilon".to_string()),
    ];

    for event in &events {
        println!("{}", describe(event));
    }
}
