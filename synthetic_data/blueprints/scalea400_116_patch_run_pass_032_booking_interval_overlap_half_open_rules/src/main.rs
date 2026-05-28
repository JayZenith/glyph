use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Booking {
    id: &'static str,
    room: &'static str,
    start: u32,
    end: u32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn can_accept(existing: &[Booking], candidate: &Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }
    existing
        .iter()
        .filter(|b| b.room == candidate.room)
        .all(|b| !overlaps(b, candidate))
}

fn main() {
    let requests = vec![
        Booking { id: "A", room: "R1", start: 540, end: 600 },
        Booking { id: "B", room: "R1", start: 600, end: 660 },
        Booking { id: "C", room: "R1", start: 590, end: 610 },
        Booking { id: "D", room: "R2", start: 590, end: 610 },
        Booking { id: "E", room: "R1", start: 660, end: 660 },
        Booking { id: "F", room: "R1", start: 660, end: 720 },
        Booking { id: "G", room: "R2", start: 610, end: 630 },
        Booking { id: "H", room: "R2", start: 600, end: 620 },
    ];

    let mut accepted = Vec::new();
    let mut room_counts: BTreeMap<&'static str, usize> = BTreeMap::new();

    for req in requests {
        if can_accept(&accepted, &req) {
            println!("accepted: {} {} {}-{}", req.id, req.room, req.start, req.end);
            *room_counts.entry(req.room).or_insert(0) += 1;
            accepted.push(req);
        } else {
            println!("rejected: {} {} {}-{}", req.id, req.room, req.start, req.end);
        }
    }

    let summary = room_counts
        .iter()
        .map(|(room, count)| format!("{}={}", room, count))
        .collect::<Vec<_>>()
        .join(",");
    println!("rooms: {}", summary);
}
