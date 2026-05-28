#[derive(Clone, Copy)]
struct Booking {
    id: u32,
    start: u32,
    end: u32,
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn conflicts(existing: &[Booking], req: Booking) -> Vec<u32> {
    existing
        .iter()
        .filter(|&&b| overlaps(b, req))
        .map(|b| b.id)
        .rev()
        .collect()
}

fn render(existing: &[Booking], reqs: &[Booking]) -> String {
    let mut lines = Vec::new();
    for &req in reqs {
        let ids = conflicts(existing, req);
        if ids.is_empty() {
            lines.push(format!("request {} -> accepted", req.id));
        } else {
            lines.push(format!("request {} -> conflicts with {:?}", req.id, ids));
        }
    }
    lines.join("\n")
}

fn main() {
    let existing = vec![
        Booking { id: 1, start: 60, end: 120 },
        Booking { id: 2, start: 150, end: 180 },
        Booking { id: 3, start: 170, end: 210 },
    ];

    let reqs = vec![
        Booking { id: 11, start: 120, end: 150 },
        Booking { id: 12, start: 155, end: 160 },
        Booking { id: 13, start: 90, end: 100 },
        Booking { id: 14, start: 175, end: 180 },
        Booking { id: 15, start: 210, end: 240 },
    ];

    println!("{}", render(&existing, &reqs));
}
