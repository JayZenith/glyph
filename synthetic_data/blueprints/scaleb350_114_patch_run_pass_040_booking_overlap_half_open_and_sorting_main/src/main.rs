use std::fmt;

#[derive(Clone, Copy)]
struct Booking {
    label: &'static str,
    start: u32,
    end: u32,
}

impl fmt::Display for Booking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    !(a.end < b.start || b.end < a.start)
}

fn classify(bookings: &[Booking]) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut accepted: Vec<Booking> = Vec::new();
    let mut rejected: Vec<&'static str> = Vec::new();

    let mut ordered = bookings.to_vec();
    ordered.sort_by_key(|b| (b.start, b.end, b.label));

    for booking in ordered {
        if booking.start > booking.end {
            rejected.push(booking.label);
            continue;
        }

        if bookings.iter().any(|existing| overlaps(existing, &booking)) {
            rejected.push(booking.label);
        } else {
            accepted.push(booking);
        }
    }

    accepted.sort_by_key(|b| b.label);
    rejected.sort();

    (
        accepted.into_iter().map(|b| b.label).collect(),
        rejected,
    )
}

fn format_labels(labels: &[&str]) -> String {
    format!("[{}]", labels.join(", "))
}

fn main() {
    let bookings = vec![
        Booking { label: "A", start: 10, end: 20 },
        Booking { label: "B", start: 20, end: 30 },
        Booking { label: "C", start: 15, end: 18 },
        Booking { label: "D", start: 30, end: 35 },
        Booking { label: "E", start: 35, end: 35 },
        Booking { label: "F", start: 34, end: 36 },
        Booking { label: "G", start: 5, end: 10 },
        Booking { label: "H", start: 8, end: 12 },
    ];

    let (accepted, rejected) = classify(&bookings);
    println!("accepted: {}", format_labels(&accepted));
    println!("rejected: {}", format_labels(&rejected));
}
