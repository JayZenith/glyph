#[derive(Clone, Copy)]
struct Booking {
    start: u32,
    end: u32,
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|&b| !overlaps(b, candidate))
}

fn main() {
    let requests = [
        Booking { start: 10, end: 12 },
        Booking { start: 12, end: 14 },
        Booking { start: 11, end: 13 },
        Booking { start: 14, end: 16 },
        Booking { start: 13, end: 15 },
    ];

    let mut accepted = Vec::new();
    let mut rejected = 0;

    for req in requests {
        if can_book(&accepted, req) {
            accepted.push(req);
        } else {
            rejected += 1;
        }
    }

    println!("accepted={} rejected={}", accepted.len(), rejected);
}
