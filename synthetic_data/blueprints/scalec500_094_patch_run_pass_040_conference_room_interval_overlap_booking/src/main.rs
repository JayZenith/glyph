use std::fmt::Write;

#[derive(Clone, Copy)]
struct Booking {
    room: &'static str,
    start: u32,
    end: u32,
}

fn parse_time(s: &str) -> u32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<u32>().unwrap() * 60 + m.parse::<u32>().unwrap()
}

fn fmt_time(m: u32) -> String {
    format!("{:02}:{:02}", m / 60, m % 60)
}

fn conflicts(existing: &Booking, room: &str, start: u32, end: u32) -> bool {
    existing.room == room && start <= existing.end && end >= existing.start
}

fn try_book(bookings: &mut Vec<Booking>, room: &'static str, start: u32, end: u32) -> bool {
    if bookings.iter().any(|b| conflicts(b, room, start, end)) {
        false
    } else {
        bookings.push(Booking { room, start, end });
        true
    }
}

fn main() {
    let requests = [
        ("RoomA", "09:00", "10:00"),
        ("RoomA", "10:00", "11:00"),
        ("RoomA", "09:30", "09:45"),
        ("RoomB", "09:15", "09:45"),
        ("RoomB", "09:45", "10:15"),
        ("RoomA", "11:00", "12:00"),
        ("RoomA", "11:30", "12:30"),
        ("RoomA", "08:00", "09:00"),
        ("RoomA", "08:30", "08:45"),
    ];

    let mut bookings = Vec::new();
    let mut out = String::new();

    for (room, s, e) in requests {
        let start = parse_time(s);
        let end = parse_time(e);
        let accepted = try_book(&mut bookings, room, start, end);
        let status = if accepted { "ACCEPT" } else { "REJECT" };
        let _ = writeln!(out, "{}-{} {} -> {}", fmt_time(start), fmt_time(end), room, status);
    }

    print!("{}", out);
}
