#[derive(Clone, Copy)]
struct Booking {
    id: &'static str,
    start: u32,
    end: u32,
}

fn parse_hhmm(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn fmt_hhmm(mins: u32) -> String {
    format!("{:02}:{:02}", mins / 60, mins % 60)
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn try_add(schedule: &mut Vec<Booking>, candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }
    if schedule.iter().any(|existing| overlaps(existing, &candidate)) {
        return false;
    }
    schedule.push(candidate);
    true
}

fn main() {
    let requests = vec![
        Booking { id: "A", start: parse_hhmm("09:00"), end: parse_hhmm("10:00") },
        Booking { id: "B", start: parse_hhmm("09:30"), end: parse_hhmm("09:45") },
        Booking { id: "C", start: parse_hhmm("10:00"), end: parse_hhmm("11:30") },
        Booking { id: "D", start: parse_hhmm("11:15"), end: parse_hhmm("12:00") },
        Booking { id: "E", start: parse_hhmm("11:30"), end: parse_hhmm("11:30") },
        Booking { id: "F", start: parse_hhmm("11:30"), end: parse_hhmm("12:00") },
        Booking { id: "G", start: parse_hhmm("08:00"), end: parse_hhmm("13:00") },
        Booking { id: "H", start: parse_hhmm("12:00"), end: parse_hhmm("13:00") },
    ];

    let mut schedule = Vec::new();
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for req in requests {
        if try_add(&mut schedule, req) {
            accepted.push(req.id);
        } else {
            rejected.push(req.id);
        }
    }

    println!("accepted: {}", accepted.join(","));
    println!("rejected: {}", rejected.join(","));
    println!("schedule:");
    for b in &schedule {
        println!("{} {}-{}", b.id, fmt_hhmm(b.start), fmt_hhmm(b.end));
    }
}
