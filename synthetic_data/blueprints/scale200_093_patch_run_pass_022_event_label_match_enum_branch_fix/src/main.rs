enum Status {
    Active,
    Inactive,
    Unknown,
}

enum Event {
    Created { name: String, status: Status },
    Updated { name: String, status: Status },
    Deleted { name: String },
}

fn render(event: &Event) -> String {
    match event {
        Event::Created { name, status } => {
            format!("created {} [{}]", name, status_tag(status))
        }
        Event::Updated { name, status } => {
            format!("updated {} [{}]", name, status_tag(status))
        }
        Event::Deleted { name } => format!("deleted {} [{}]", name, status_tag(&Status::Active)),
    }
}

fn status_tag(status: &Status) -> &'static str {
    match status {
        Status::Active => "active",
        Status::Inactive => "inactive",
        Status::Unknown => "inactive",
    }
}

fn main() {
    let events = vec![
        Event::Created {
            name: "alpha".to_string(),
            status: Status::Active,
        },
        Event::Updated {
            name: "beta".to_string(),
            status: Status::Inactive,
        },
        Event::Deleted {
            name: "gamma".to_string(),
        },
    ];

    for event in &events {
        println!("{}", render(event));
    }
}
