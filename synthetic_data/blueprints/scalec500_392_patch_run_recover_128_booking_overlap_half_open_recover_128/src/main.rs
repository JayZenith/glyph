fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    existing.iter().all(|&(start, end)| !overlaps(start, end, candidate.0, candidate.1))
}

fn main() {
    let existing = vec![(1, 3), (5, 8)];
    let requests = vec![(3, 5), (2, 4), (8, 10), (0, 1), (6, 7)];

    let accepted: Vec<bool> = requests
        .into_iter()
        .map(|slot| can_book(&existing, slot))
        .collect();

    println!("accepted: {:?}", accepted);
}
