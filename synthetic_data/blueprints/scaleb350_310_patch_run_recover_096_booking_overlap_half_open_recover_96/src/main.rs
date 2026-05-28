use std::fmt::Write;

#[derive(Clone, Copy)]
struct Booking {
    room: char,
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
    if a.room != b.room {
        return false;
    }
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let requests = vec![
        Booking { room: 'A', start: parse_hhmm("09:00"), end: parse_hhmm("10:00") },
        Booking { room: 'A', start: parse_hhmm("10:00"), end: parse_hhmm("11:00") },
        Booking { room: 'A', start: parse_hhmm("09:30"), end: parse_hhmm("09:45") },
        Booking { room: 'B', start: parse_hhmm("09:15"), end: parse_hhmm("09:45") },
        Booking { room: 'A', start: parse_hhmm("11:00"), end: parse_hhmm("12:00") },
        Booking { room: 'B', start: parse_hhmm("10:30"), end: parse_hhmm("11:00") },
        Booking { room: 'B', start: parse_hhmm("10:45"), end: parse_hhmm("11:15") },
    ];

    let mut accepted: Vec<Booking> = Vec::new();
    let mut out = String::new();

    for req in requests {
        let conflict = accepted.iter().any(|b| overlaps(b, &req));
        if conflict {
            let _ = writeln!(
                out,
                "rejected {}-{} room {}",
                fmt_hhmm(req.start),
                fmt_hhmm(req.end),
                req.room
            );
        } else {
            accepted.push(req);
            let _ = writeln!(
                out,
                "accepted {}-{} room {}",
                fmt_hhmm(req.start),
                fmt_hhmm(req.end),
                req.room
            );
        }
    }

    accepted.sort_by_key(|b| b.start);
    let _ = writeln!(out, "final:");
    for b in accepted {
        let _ = writeln!(
            out,
            "{} {}-{}",
            b.room,
            fmt_hhmm(b.start),
            fmt_hhmm(b.end)
        );
    }

    print!("{}", out);
}
