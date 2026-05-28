#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| candidate.start <= b.end && b.start <= candidate.end)
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    !has_conflict(existing, candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_bookings_do_not_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
    }

    #[test]
    fn overlapping_bookings_conflict() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(has_conflict(&existing, Booking::new(15, 18)));
        assert!(has_conflict(&existing, Booking::new(18, 35)));
        assert!(!has_conflict(&existing, Booking::new(20, 30)));
    }

    #[test]
    fn invalid_candidate_range_cannot_be_booked() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(25, 25)));
        assert!(!can_book(&existing, Booking::new(30, 10)));
    }
}
