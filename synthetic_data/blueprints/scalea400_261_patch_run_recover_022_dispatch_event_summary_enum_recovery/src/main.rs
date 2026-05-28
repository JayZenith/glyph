enum Event {
    Create { target: &'static str, urgent: bool },
    Dispatch { channel: Channel, dry_run: bool },
    Finalize(Outcome),
}

enum Channel {
    Cli,
    Api,
}

enum Outcome {
    Shipped { item: &'static str, automated: bool },
    Billed { doc: &'static str, paid: bool },
    Archived { kind: &'static str },
    Notified { audience: &'static str, success: bool },
}

fn describe(event: &Event) -> String {
    match event {
        Event::Create { target, urgent } => {
            let mode = if *urgent { "auto" } else { "manual" };
            format!("create {target} via {mode}")
        }
        Event::Dispatch { channel, dry_run } => {
            let ch = match channel {
                Channel::Cli => "cli",
                Channel::Api => "api",
            };
            let state = if *dry_run { "skipped" } else { "sent" };
            format!("dispatch {ch} {state}")
        }
        Event::Finalize(outcome) => match outcome {
            Outcome::Shipped { item, automated } => {
                let mode = if *automated { "manual" } else { "auto" };
                format!("ship {item} via {mode}")
            }
            Outcome::Billed { doc, paid } => {
                let mode = if *paid { "manual" } else { "auto" };
                format!("bill {doc} via {mode}")
            }
            Outcome::Archived { kind } => format!("archive {kind} done"),
            Outcome::Notified { audience, success } => {
                let state = if *success { "skipped" } else { "sent" };
                format!("notify {audience} {state}")
            }
        },
    }
}

fn main() {
    let events = [
        Event::Create {
            target: "report",
            urgent: false,
        },
        Event::Finalize(Outcome::Shipped {
            item: "package",
            automated: true,
        }),
        Event::Finalize(Outcome::Billed {
            doc: "invoice",
            paid: true,
        }),
        Event::Finalize(Outcome::Archived { kind: "logs" }),
        Event::Finalize(Outcome::Notified {
            audience: "ops",
            success: true,
        }),
    ];

    for event in &events {
        println!("{}", describe(event));
    }
}
