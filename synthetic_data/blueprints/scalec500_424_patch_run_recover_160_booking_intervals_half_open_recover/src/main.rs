struct Booking {
    label: &'static str,
    start: u32,
    end: u32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let requests = vec![
        Booking { label: "A", start: 540, end: 600 },
        Booking { label: "B", start: 590, end: 610 },
        Booking { label: "C", start: 600, end: 630 },
        Booking { label: "D", start: 630, end: 630 },
        Booking { label: "E", start: 630, end: 660 },
        Booking { label: "F", start: 620, end: 625 },
    ];

    let mut accepted: Vec<Booking> = Vec::new();
    let mut rejected: Vec<(&'static str, &'static str)> = Vec::new();

    for req in requests {
        if accepted.iter().any(|a| overlaps(a, &req)) {
            rejected.push((req.label, "overlap"));
        } else {
            accepted.push(req);
        }
    }

    accepted.sort_by_key(|b| b.end);

    println!("accepted:");
    for b in &accepted {
        println!("{} {}-{}", b.label, b.start, b.end);
    }
    println!("rejected:");
    for (label, reason) in &rejected {
        println!("{} {}", label, reason);
    }
}
