fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

fn conflicting_ids(existing: &[(u32, u32, u32)], request: (u32, u32)) -> Vec<u32> {
    let mut ids = Vec::new();
    for &(id, start, end) in existing {
        if overlaps(start, end, request.0, request.1) {
            ids.push(id);
        }
    }
    ids
}

fn main() {
    let existing = vec![
        (1, 9, 11),
        (2, 11, 13),
        (3, 14, 16),
        (4, 12, 15),
    ];
    let request = (11, 14);
    let conflicts = conflicting_ids(&existing, request);
    println!("Conflicts: {:?}", conflicts);
}
