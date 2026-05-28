#[derive(Clone, Copy)]
struct Booking {
    id: u32,
    room: char,
    start: u32,
    end: u32,
}

fn conflicts(existing: &[Booking], req: Booking) -> Vec<u32> {
    let mut ids = Vec::new();
    for b in existing {
        if b.start <= req.end && req.start <= b.end {
            ids.push(b.id);
        }
    }
    ids
}

fn describe(existing: &[Booking], req: Booking) -> String {
    let hits = conflicts(existing, req);
    if hits.is_empty() {
        format!(
            "request #{} room {} [{}, {}): available",
            req.id, req.room, req.start, req.end
        )
    } else {
        format!(
            "request #{} room {} [{}, {}): conflict with {:?}",
            req.id, req.room, req.start, req.end, hits
        )
    }
}

fn main() {
    let existing = vec![
        Booking { id: 1, room: 'A', start: 9, end: 12 },
        Booking { id: 2, room: 'A', start: 12, end: 15 },
        Booking { id: 3, room: 'A', start: 13, end: 14 },
        Booking { id: 4, room: 'B', start: 10, end: 12 },
        Booking { id: 5, room: 'B', start: 14, end: 18 },
    ];

    let requests = [
        Booking { id: 100, room: 'A', start: 12, end: 14 },
        Booking { id: 101, room: 'A', start: 15, end: 17 },
        Booking { id: 102, room: 'B', start: 12, end: 16 },
        Booking { id: 103, room: 'A', start: 20, end: 20 },
        Booking { id: 104, room: 'C', start: 8, end: 9 },
    ];

    for req in requests {
        println!("{}", describe(&existing, req));
    }
}
