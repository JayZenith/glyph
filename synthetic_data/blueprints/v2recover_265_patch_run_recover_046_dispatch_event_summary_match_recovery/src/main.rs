enum Event {
    Ingest { file: &'static str, urgent: bool },
    Notify { channel: &'static str, message: &'static str, urgent: bool },
    Retry { job: &'static str, attempts: u8 },
    Archive { path: &'static str, compress: bool },
    Skip { reason: &'static str },
}

#[derive(Clone, Copy)]
enum Bucket {
    Immediate,
    Queued,
    Deferred,
}

fn bucket_for(event: &Event) -> Bucket {
    match event {
        Event::Ingest { urgent, .. } => {
            if *urgent { Bucket::Queued } else { Bucket::Deferred }
        }
        Event::Notify { urgent, .. } => {
            if *urgent { Bucket::Immediate } else { Bucket::Deferred }
        }
        Event::Retry { attempts, .. } => {
            if *attempts >= 3 { Bucket::Immediate } else { Bucket::Deferred }
        }
        Event::Archive { compress, .. } => {
            if *compress { Bucket::Queued } else { Bucket::Immediate }
        }
        Event::Skip { .. } => Bucket::Immediate,
    }
}

fn describe(event: &Event) -> String {
    match event {
        Event::Ingest { file, .. } => format!("ingest({})", file),
        Event::Notify { channel, message, .. } => format!("notify({}: {})", channel, message),
        Event::Retry { job, attempts } => format!("retry({}, {})", job, attempts),
        Event::Archive { path, compress } => {
            if *compress {
                format!("archive({}.gz)", path)
            } else {
                format!("archive({})", path)
            }
        }
        Event::Skip { reason } => format!("skip({})", reason),
    }
}

fn main() {
    let events = [
        Event::Ingest { file: "data.csv", urgent: true },
        Event::Notify { channel: "ops", message: "import started", urgent: false },
        Event::Retry { job: "job42", attempts: 3 },
        Event::Archive { path: "/tmp/out", compress: true },
        Event::Notify { channel: "admin", message: "high memory", urgent: true },
        Event::Skip { reason: "debug mode" },
        Event::Archive { path: "/var/log/app", compress: false },
        Event::Retry { job: "job99", attempts: 1 },
        Event::Ingest { file: "delta.bin", urgent: false },
    ];

    let mut immediate = Vec::new();
    let mut queued = Vec::new();
    let mut deferred = Vec::new();

    for event in &events {
        let text = describe(event);
        match bucket_for(event) {
            Bucket::Immediate => immediate.push(text),
            Bucket::Queued => queued.push(text),
            Bucket::Deferred => deferred.push(text),
        }
    }

    println!("queued: {}", queued.join(", "));
    println!("immediate: {}", immediate.join(", "));
    println!("deferred: {}", deferred.join(", "));
}
