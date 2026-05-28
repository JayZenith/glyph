fn parse_hhmm(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

fn line(a_start: &str, a_end: &str, b_start: &str, b_end: &str) -> String {
    let conflict = overlaps(
        parse_hhmm(a_start),
        parse_hhmm(a_end),
        parse_hhmm(b_start),
        parse_hhmm(b_end),
    );

    format!(
        "{}-{} vs {}-{} => {}",
        a_start,
        a_end,
        b_start,
        b_end,
        if conflict { "conflict" } else { "ok" }
    )
}

fn main() {
    let cases = [
        ("09:00", "10:00", "10:00", "11:00"),
        ("09:00", "10:00", "09:30", "09:45"),
        ("13:00", "15:00", "12:00", "13:00"),
        ("13:00", "15:00", "14:59", "16:00"),
        ("08:00", "09:00", "07:00", "08:30"),
    ];

    for (a_start, a_end, b_start, b_end) in cases {
        println!("{}", line(a_start, a_end, b_start, b_end));
    }
}
