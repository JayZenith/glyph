#[derive(Clone, Copy)]
struct Booking {
    room: char,
    start: u32,
    end: u32,
}

struct Request {
    room: char,
    start: u32,
    end: u32,
    name: &'static str,
}

fn parse_hhmm(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn fmt_hhmm(mins: u32) -> String {
    format!("{:02}:{:02}", mins / 60, mins % 60)
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let requests = vec![
        Request { room: 'A', start: parse_hhmm("09:00"), end: parse_hhmm("10:00"), name: "alpha" },
        Request { room: 'A', start: parse_hhmm("09:30"), end: parse_hhmm("09:45"), name: "beta" },
        Request { room: 'A', start: parse_hhmm("10:00"), end: parse_hhmm("11:00"), name: "alpha" },
        Request { room: 'B', start: parse_hhmm("09:30"), end: parse_hhmm("10:30"), name: "gamma" },
        Request { room: 'A', start: parse_hhmm("08:50"), end: parse_hhmm("09:10"), name: "delta" },
        Request { room: 'A', start: parse_hhmm("11:00"), end: parse_hhmm("12:00"), name: "epsilon" },
        Request { room: 'B', start: parse_hhmm("10:15"), end: parse_hhmm("10:45"), name: "zeta" },
        Request { room: 'A', start: parse_hhmm("11:30"), end: parse_hhmm("11:45"), name: "eta" },
    ];

    let mut accepted: Vec<(Request, Booking)> = Vec::new();
    let mut rejected: Vec<String> = Vec::new();

    for req in requests {
        let candidate = Booking {
            room: req.room,
            start: req.start,
            end: req.end,
        };

        let conflict = accepted
            .iter()
            .find(|(_, existing)| overlaps(candidate, *existing));

        if let Some((prior, _)) = conflict {
            rejected.push(format!("{} {} overlaps {}", req.room, req.name, prior.name));
        } else {
            accepted.push((req, candidate));
        }
    }

    println!("accepted:");
    for (req, booking) in &accepted {
        println!(
            "{} {} {}-{}",
            req.room,
            req.name,
            fmt_hhmm(booking.start),
            fmt_hhmm(booking.end)
        );
    }

    println!("rejected:");
    for line in &rejected {
        println!("{}", line);
    }
}
