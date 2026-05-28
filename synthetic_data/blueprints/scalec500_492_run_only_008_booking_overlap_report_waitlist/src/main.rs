struct Booking {
    id: &'static str,
    start: u32,
    end: u32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start < b.end && b.start < a.end
}

fn main() {
    let requests = [
        Booking { id: "A", start: 540, end: 600 },
        Booking { id: "B", start: 570, end: 630 },
        Booking { id: "C", start: 600, end: 660 },
        Booking { id: "D", start: 660, end: 660 },
        Booking { id: "E", start: 660, end: 720 },
        Booking { id: "F", start: 700, end: 710 },
        Booking { id: "G", start: 720, end: 780 },
        Booking { id: "H", start: 719, end: 720 },
    ];

    let mut accepted: Vec<&Booking> = Vec::new();
    let mut rejected_ids: Vec<&str> = Vec::new();

    for req in &requests {
        let valid = req.start < req.end;
        let conflict = accepted.iter().any(|booked| overlaps(req, booked));
        if valid && !conflict {
            accepted.push(req);
        } else {
            rejected_ids.push(req.id);
        }
    }

    let accepted_ids = accepted.iter().map(|b| b.id).collect::<Vec<_>>().join(",");
    let rejected_ids_joined = rejected_ids.join(",");

    println!("accepted: {}", accepted.len());
    println!("rejected: {}", rejected_ids.len());
    println!("accepted_ids: {}", accepted_ids);
    print!("rejected_ids: {}", rejected_ids_joined);
}
