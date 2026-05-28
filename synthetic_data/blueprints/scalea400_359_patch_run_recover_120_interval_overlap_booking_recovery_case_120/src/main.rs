#[derive(Clone, Copy)]
struct Booking {
    id: u32,
    room: char,
    start: u32,
    end: u32,
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let requests = [
        Booking { id: 1, room: 'A', start: 9, end: 11 },
        Booking { id: 2, room: 'A', start: 10, end: 12 },
        Booking { id: 3, room: 'A', start: 11, end: 13 },
        Booking { id: 4, room: 'A', start: 14, end: 16 },
        Booking { id: 5, room: 'A', start: 15, end: 16 },
        Booking { id: 6, room: 'B', start: 9, end: 10 },
        Booking { id: 7, room: 'B', start: 9, end: 10 },
        Booking { id: 8, room: 'A', start: 13, end: 13 },
    ];

    let mut accepted: Vec<Booking> = Vec::new();
    let mut conflicts: Vec<String> = Vec::new();

    for req in requests {
        let hit = accepted.iter().find(|&&b| overlaps(req, b));
        if let Some(prev) = hit {
            conflicts.push(format!("{}#{} overlaps {}#{}", req.room, req.id, prev.room, prev.id));
        } else {
            accepted.push(req);
        }
    }

    println!("conflicts:");
    for line in conflicts {
        println!("{}", line);
    }
}
