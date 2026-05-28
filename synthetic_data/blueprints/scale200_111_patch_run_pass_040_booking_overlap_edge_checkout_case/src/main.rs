fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    !(a.1 < b.0 || b.1 < a.0)
}

fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    if candidate.0 >= candidate.1 {
        return false;
    }
    !existing.iter().any(|&slot| overlaps(slot, candidate))
}

fn main() {
    let existing = [(1, 3), (5, 8)];
    let requests = [
        ("A", (3, 5)),
        ("B", (2, 4)),
        ("C", (8, 10)),
        ("D", (10, 12)),
        ("E", (7, 9)),
    ];

    for (name, slot) in requests {
        let status = if can_book(&existing, slot) {
            "accepted"
        } else {
            "conflict"
        };
        println!("{}: {}", name, status);
    }
}
