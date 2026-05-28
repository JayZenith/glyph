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

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    existing.iter().all(|&b| !overlaps(b, requested))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_to_back_bookings_are_allowed() {
        let existing = [Booking::new(10, 12)];
        assert!(can_book(&existing, Booking::new(12, 15)));
    }

    #[test]
    fn overlapping_middle_day_is_rejected() {
        let existing = [Booking::new(10, 12)];
        assert!(!can_book(&existing, Booking::new(11, 13)));
    }

    #[test]
    fn contained_booking_is_rejected() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 15)));
    }

    #[test]
    fn invalid_requested_interval_is_rejected() {
        let existing = [Booking::new(20, 25)];
        assert!(!can_book(&existing, Booking::new(30, 30)));
        assert!(!can_book(&existing, Booking::new(31, 29)));
    }
}
