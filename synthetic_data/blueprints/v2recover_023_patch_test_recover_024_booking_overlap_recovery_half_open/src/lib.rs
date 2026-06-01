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

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| !overlaps(*b, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 30)));
    }

    #[test]
    fn contained_interval_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 15)));
    }

    #[test]
    fn zero_length_booking_is_invalid() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(25, 25)));
    }

    #[test]
    fn reversed_booking_is_invalid() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(30, 25)));
    }

    #[test]
    fn checks_against_all_existing_bookings() {
        let existing = [Booking::new(0, 5), Booking::new(8, 10)];
        assert!(!can_book(&existing, Booking::new(4, 9)));
    }
}
