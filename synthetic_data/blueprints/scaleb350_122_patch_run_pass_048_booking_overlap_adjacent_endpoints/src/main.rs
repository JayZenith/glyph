fn parse_hhmm(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

#[derive(Clone, Copy)]
struct Booking<'a> {
    name: &'a str,
    start: u32,
    end: u32,
}

fn overlaps(a: Booking<'_>, b: Booking<'_>) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let requests = [
        ("A", "09:00", "10:00"),
        ("B", "10:00", "11:30"),
        ("C", "11:00", "12:00"),
        ("D", "11:30", "12:00"),
        ("E", "09:30", "09:45"),
    ];

    let mut accepted: Vec<Booking<'_>> = Vec::new();
    let mut rejected: Vec<String> = Vec::new();

    for (name, start, end) in requests {
        let booking = Booking {
            name,
            start: parse_hhmm(start),
            end: parse_hhmm(end),
        };

        if let Some(conflict) = accepted.iter().copied().find(|b| overlaps(*b, booking)) {
            rejected.push(format!("{} conflicts with {}", booking.name, conflict.name));
        } else {
            accepted.push(booking);
        }
    }

    println!("accepted:");
    for b in &accepted {
        println!("{} {:02}:{:02}-{:02}:{:02}", b.name, b.start / 60, b.start % 60, b.end / 60, b.end % 60);
    }
    println!("rejected:");
    for line in &rejected {
        println!("{}", line);
    }
}
