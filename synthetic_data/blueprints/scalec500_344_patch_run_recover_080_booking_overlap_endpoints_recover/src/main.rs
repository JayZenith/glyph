fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn main() {
    let checks = [
        ((9, 11), (10, 12)),
        ((12, 14), (14, 16)),
        ((14, 18), (13, 15)),
        ((8, 9), (9, 10)),
    ];

    for (a, b) in checks {
        let status = if overlaps(a, b) { "conflict" } else { "ok" };
        println!(
            "{:02}-{:02} vs {:02}-{:02} => {}",
            a.0, a.1, b.0, b.1, status
        );
    }
}
