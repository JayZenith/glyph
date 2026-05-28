use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
struct Booking {
    room: &'static str,
    id: &'static str,
    start: u32,
    end: u32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let bookings = vec![
        Booking { room: "A", id: "A1", start: 9, end: 11 },
        Booking { room: "A", id: "A2", start: 11, end: 12 },
        Booking { room: "A", id: "A3", start: 10, end: 13 },
        Booking { room: "B", id: "B1", start: 8, end: 10 },
        Booking { room: "B", id: "B2", start: 10, end: 12 },
        Booking { room: "B", id: "B3", start: 9, end: 10 },
        Booking { room: "C", id: "C5", start: 14, end: 14 },
    ];

    let mut invalid = Vec::new();
    let mut conflicts: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();

    for i in 0..bookings.len() {
        let a = &bookings[i];
        if a.start > a.end {
            invalid.push(a.id);
        }
        for j in (i + 1)..bookings.len() {
            let b = &bookings[j];
            if overlaps(a, b) {
                conflicts
                    .entry(a.room)
                    .or_default()
                    .insert(format!("{}-{}", a.id, b.id));
            }
        }
    }

    println!("invalid bookings: {}", invalid.join(", "));
    for (room, pairs) in conflicts {
        let joined = pairs.into_iter().collect::<Vec<_>>().join(", ");
        println!("{}: {}", room, joined);
    }
}
