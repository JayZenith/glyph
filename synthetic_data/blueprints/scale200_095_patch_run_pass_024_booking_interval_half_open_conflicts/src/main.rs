use std::fmt::Write;

#[derive(Clone, Debug)]
struct Booking {
    room: &'static str,
    start: u32,
    end: u32,
}

#[derive(Clone, Debug)]
struct Request {
    id: &'static str,
    room: &'static str,
    start: u32,
    end: u32,
}

fn parse_time(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn fmt_time(mins: u32) -> String {
    format!("{:02}:{:02}", mins / 60, mins % 60)
}

fn overlaps(a: &Booking, b: &Request) -> bool {
    if a.room != b.room {
        return true;
    }
    !(a.end < b.start || b.end < a.start)
}

fn can_book(existing: &[Booking], req: &Request) -> bool {
    if req.start > req.end {
        return false;
    }
    existing.iter().all(|b| !overlaps(b, req))
}

fn main() {
    let mut existing = vec![
        Booking { room: "ROOM1", start: parse_time("09:00"), end: parse_time("10:00") },
        Booking { room: "ROOM1", start: parse_time("13:00"), end: parse_time("14:00") },
        Booking { room: "ROOM2", start: parse_time("09:30"), end: parse_time("10:30") },
    ];

    let requests = vec![
        Request { id: "A", room: "ROOM1", start: parse_time("09:00"), end: parse_time("10:00") },
        Request { id: "B", room: "ROOM1", start: parse_time("10:00"), end: parse_time("11:00") },
        Request { id: "C", room: "ROOM1", start: parse_time("09:30"), end: parse_time("09:45") },
        Request { id: "D", room: "ROOM2", start: parse_time("09:30"), end: parse_time("10:30") },
        Request { id: "E", room: "ROOM2", start: parse_time("08:00"), end: parse_time("09:00") },
        Request { id: "F", room: "ROOM2", start: parse_time("10:00"), end: parse_time("11:00") },
        Request { id: "G", room: "ROOM1", start: parse_time("11:00"), end: parse_time("12:00") },
        Request { id: "H", room: "ROOM1", start: parse_time("08:30"), end: parse_time("09:00") },
    ];

    let mut out = String::new();

    for req in requests {
        let ok = can_book(&existing, &req);
        let status = if ok { "BOOKED" } else { "REJECT" };
        let _ = writeln!(
            out,
            "{} {} {}-{} => {}",
            req.id,
            req.room,
            fmt_time(req.start),
            fmt_time(req.end),
            status
        );
        if ok {
            existing.push(Booking {
                room: req.room,
                start: req.start,
                end: req.end,
            });
        }
    }

    print!("{}", out.trim_end());
}
