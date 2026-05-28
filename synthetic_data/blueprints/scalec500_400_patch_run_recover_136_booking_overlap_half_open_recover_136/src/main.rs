use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Booking<'a> {
    room: &'a str,
    start: u32,
    end: u32,
}

fn fmt_time(m: u32) -> String {
    format!("{:02}:{:02}", m / 60, m % 60)
}

fn fmt_booking(b: &Booking<'_>) -> String {
    format!("{}-{}", fmt_time(b.start), fmt_time(b.end))
}

fn conflicts(existing: &[Booking<'_>], cand: &Booking<'_>) -> bool {
    existing.iter().any(|b| cand.start <= b.end && b.start <= cand.end)
}

fn main() {
    let requests = [
        Booking { room: "alpha", start: 9 * 60, end: 10 * 60 },
        Booking { room: "alpha", start: 10 * 60, end: 11 * 60 },
        Booking { room: "alpha", start: 9 * 60 + 30, end: 9 * 60 + 45 },
        Booking { room: "beta", start: 9 * 60 + 30, end: 10 * 60 + 30 },
        Booking { room: "beta", start: 10 * 60 + 30, end: 11 * 60 },
        Booking { room: "beta", start: 10 * 60 + 15, end: 10 * 60 + 45 },
        Booking { room: "gamma", start: 8 * 60, end: 9 * 60 },
        Booking { room: "gamma", start: 8 * 60 + 30, end: 8 * 60 + 45 },
        Booking { room: "gamma", start: 9 * 60, end: 9 * 60 + 15 },
    ];

    let mut accepted: BTreeMap<&str, Vec<Booking>> = BTreeMap::new();
    let mut lines = Vec::new();

    for req in requests {
        let room_bookings = accepted.values().next().map(Vec::as_slice).unwrap_or(&[]);
        let ok = !conflicts(room_bookings, &req);
        lines.push(format!(
            "{} {} -> {}",
            req.room,
            fmt_booking(&req),
            if ok { "accepted" } else { "rejected" }
        ));
        if ok {
            accepted.entry(req.room).or_default().push(req);
        }
    }

    for (room, items) in &accepted {
        let joined = items.iter().map(fmt_booking).collect::<Vec<_>>().join(", ");
        lines.push(format!("{}: {}", room, joined));
    }

    println!("{}", lines.join("\n"));
}
