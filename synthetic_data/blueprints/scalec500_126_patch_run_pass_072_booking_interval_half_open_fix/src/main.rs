fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    !(a.1 < b.0 || b.1 < a.0)
}

fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    candidate.0 < candidate.1 && existing.iter().all(|&slot| !overlaps(slot, candidate))
}

fn fmt(slot: (u32, u32)) -> String {
    format!("{:02}-{:02}", slot.0, slot.1)
}

fn main() {
    let requests = [
        (9, 11),
        (11, 12),
        (10, 12),
        (12, 14),
        (8, 9),
        (14, 16),
        (13, 15),
    ];

    let mut booked = Vec::new();
    let mut out = Vec::new();

    for req in requests {
        let accepted = can_book(&booked, req);
        out.push(format!(
            "{} => {}",
            fmt(req),
            if accepted { "accepted" } else { "rejected" }
        ));
        if accepted {
            booked.push(req);
        }
    }

    print!("{}", out.join("\n"));
}
