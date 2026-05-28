fn parse_hhmm(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn fmt_hhmm(mins: u32) -> String {
    format!("{:02}:{:02}", mins / 60, mins % 60)
}

#[derive(Clone, Copy)]
struct Booking {
    start: u32,
    end: u32,
}

impl Booking {
    fn label(&self) -> String {
        format!("{}-{}", fmt_hhmm(self.start), fmt_hhmm(self.end))
    }
}

fn overlaps(a: Booking, b: Booking) -> bool {
    !(a.end < b.start || b.end < a.start)
}

fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }
    !existing.iter().any(|&b| overlaps(b, candidate))
}

fn main() {
    let requests = [
        Booking { start: parse_hhmm("09:00"), end: parse_hhmm("10:00") },
        Booking { start: parse_hhmm("10:00"), end: parse_hhmm("11:00") },
        Booking { start: parse_hhmm("09:30"), end: parse_hhmm("09:45") },
        Booking { start: parse_hhmm("11:00"), end: parse_hhmm("12:30") },
        Booking { start: parse_hhmm("12:30"), end: parse_hhmm("13:00") },
        Booking { start: parse_hhmm("12:00"), end: parse_hhmm("12:30") },
        Booking { start: parse_hhmm("08:00"), end: parse_hhmm("09:00") },
        Booking { start: parse_hhmm("08:30"), end: parse_hhmm("09:15") },
        Booking { start: parse_hhmm("13:00"), end: parse_hhmm("13:00") },
        Booking { start: parse_hhmm("10:15"), end: parse_hhmm("10:45") },
    ];

    let mut accepted = Vec::new();
    let mut rejected = 0;

    for req in requests {
        if can_book(&accepted, req) {
            println!("{} => accepted", req.label());
            accepted.push(req);
        } else {
            println!("{} => rejected", req.label());
            rejected += 1;
        }
    }

    println!("accepted={} rejected={}", accepted.len(), rejected);
}
