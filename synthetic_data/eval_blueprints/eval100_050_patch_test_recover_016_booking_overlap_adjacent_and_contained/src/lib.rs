#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start < end);
        Self { start, end }
    }
}

pub fn conflicts(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| {
        (candidate.start >= b.start && candidate.start <= b.end)
            || (candidate.end >= b.start && candidate.end <= b.end)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_bookings_do_not_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(!conflicts(&existing, Booking::new(20, 25)));
        assert!(!conflicts(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn partial_overlap_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(15, 25)));
        assert!(conflicts(&existing, Booking::new(5, 15)));
    }

    #[test]
    fn containing_existing_booking_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn separate_bookings_do_not_conflict() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!conflicts(&existing, Booking::new(20, 30)));
        assert!(!conflicts(&existing, Booking::new(40, 50)));
        assert!(!conflicts(&existing, Booking::new(0, 5)));
    }
}
