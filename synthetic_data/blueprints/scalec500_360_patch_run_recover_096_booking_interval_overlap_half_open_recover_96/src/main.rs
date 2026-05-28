#[derive(Clone, Copy)]
struct Booking {
    start: u32,
    end: u32,
    label: &'static str,
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    !existing.iter().any(|&b| overlaps(b, candidate))
}

fn main() {
    let requests = [
        Booking { start: 10, end: 12, label: "A" },
        Booking { start: 12, end: 14, label: "B" },
        Booking { start: 11, end: 13, label: "C" },
        Booking { start: 14, end: 16, label: "D" },
        Booking { start: 9, end: 10, label: "E" },
    ];

    let mut accepted = Vec::new();
    for req in requests {
        if can_book(&accepted, req) {
            println!("accepted {}", req.label);
            accepted.push(req);
        } else {
            println!("rejected {}", req.label);
        }
    }
}
