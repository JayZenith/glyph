fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn fmt_time(n: u32) -> String {
    format!("{:02}{:02}", n / 60, n % 60)
}

fn fmt_slot(slot: (u32, u32)) -> String {
    format!("{}-{}", fmt_time(slot.0), fmt_time(slot.1))
}

fn main() {
    let requests = [
        (540, 600),
        (600, 630),
        (570, 585),
        (630, 660),
        (625, 640),
        (660, 690),
    ];

    let mut booked: Vec<(u32, u32)> = Vec::new();
    let mut out = Vec::new();

    for req in requests {
        let conflicts: Vec<String> = booked
            .iter()
            .copied()
            .filter(|&slot| overlaps(req, slot))
            .take(1)
            .map(fmt_slot)
            .collect();

        if conflicts.is_empty() {
            booked.push(req);
            out.push(format!("{} OK", fmt_slot(req)));
        } else {
            out.push(format!("{} CONFLICT with {}", fmt_slot(req), conflicts.join(",")));
        }
    }

    println!("{}", out.join("\n"));
}
