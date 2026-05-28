#[derive(Clone, Copy)]
struct Booking {
    id: &'static str,
    start: i32,
    end: i32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let bookings = vec![
        Booking { id: "A", start: 9, end: 11 },
        Booking { id: "B", start: 10, end: 12 },
        Booking { id: "C", start: 12, end: 13 },
        Booking { id: "D", start: 12, end: 14 },
        Booking { id: "E", start: 15, end: 18 },
        Booking { id: "F", start: 20, end: 20 },
        Booking { id: "G", start: 18, end: 20 },
        Booking { id: "H", start: 19, end: 20 },
    ];

    let mut accepted: Vec<&str> = Vec::new();
    let mut accepted_bookings: Vec<Booking> = Vec::new();
    let mut conflicts: Vec<String> = Vec::new();
    let mut invalid: Vec<&str> = Vec::new();

    for b in bookings {
        if b.start > b.end {
            invalid.push(b.id);
            continue;
        }

        let mut hit = None;
        for prev in &accepted_bookings {
            if overlaps(&b, prev) {
                hit = Some(prev.id);
                break;
            }
        }

        if let Some(prev_id) = hit {
            conflicts.push(format!("{}-{}", b.id, prev_id));
        } else {
            accepted.push(b.id);
            accepted_bookings.push(b);
        }
    }

    println!("accepted:[{}]", accepted.join(","));
    println!("conflicts:[{}]", conflicts.join(","));
    println!("invalid:[{}]", invalid.join(","));
}
