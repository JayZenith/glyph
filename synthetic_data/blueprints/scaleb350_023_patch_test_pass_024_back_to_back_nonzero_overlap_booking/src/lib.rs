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

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    if requested.start > requested.end {
        return false;
    }

    for booking in existing {
        if requested.start <= booking.end && booking.start <= requested.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_length_and_inverted_requests() {
        let existing = [Booking::new(10, 12)];
        assert!(!can_book(&existing, Booking::new(8, 8)));
        assert!(!can_book(&existing, Booking::new(9, 7)));
    }

    #[test]
    fn allows_back_to_back_before_and_after() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(5, 10)));
        assert!(can_book(&existing, Booking::new(20, 25)));
    }

    #[test]
    fn rejects_partial_overlap_on_left_and_right() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(9, 11)));
        assert!(!can_book(&existing, Booking::new(19, 22)));
    }

    #[test]
    fn rejects_containment_in_both_directions() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(8, 22)));
    }

    #[test]
    fn checks_against_multiple_existing_bookings() {
        let existing = [Booking::new(2, 4), Booking::new(8, 10), Booking::new(15, 18)];
        assert!(can_book(&existing, Booking::new(4, 8)));
        assert!(!can_book(&existing, Booking::new(3, 9)));
        assert!(!can_book(&existing, Booking::new(10, 16)));
    }
}
