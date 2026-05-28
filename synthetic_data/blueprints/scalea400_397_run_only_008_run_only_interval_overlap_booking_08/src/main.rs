fn parse_hm(s: &str) -> Option<i32> {
    let (h, m) = s.split_once(':')?;
    let h: i32 = h.parse().ok()?;
    let m: i32 = m.parse().ok()?;
    if !(0..24).contains(&h) || !(0..60).contains(&m) {
        return None;
    }
    Some(h * 60 + m)
}

fn overlaps(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn booking_status(existing: &[(&str, &str)], request: (&str, &str)) -> &'static str {
    let req = match (parse_hm(request.0), parse_hm(request.1)) {
        (Some(s), Some(e)) if s < e => (s, e),
        _ => return "INVALID",
    };

    for &(s, e) in existing {
        let slot = match (parse_hm(s), parse_hm(e)) {
            (Some(a), Some(b)) if a < b => (a, b),
            _ => return "INVALID",
        };
        if overlaps(slot, req) {
            return "CONFLICT";
        }
    }
    "OK"
}

fn main() {
    let rooms = [
        (
            "room-A",
            vec![("09:00", "10:30"), ("11:00", "12:00")],
            ("10:15", "11:15"),
        ),
        (
            "room-B",
            vec![("09:00", "10:00"), ("10:00", "11:00")],
            ("11:00", "11:30"),
        ),
        (
            "room-C",
            vec![("08:00", "09:00"), ("13:00", "12:00")],
            ("09:30", "10:00"),
        ),
    ];

    let mut out = Vec::new();
    for (name, existing, request) in rooms {
        out.push(format!("{}: {}", name, booking_status(&existing, request)));
    }
    print!("{}", out.join("\n"));
}
