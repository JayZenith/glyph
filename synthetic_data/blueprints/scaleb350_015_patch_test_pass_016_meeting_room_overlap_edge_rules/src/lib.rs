#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            cancelled: false,
        }
    }

    pub fn cancelled(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            cancelled: true,
        }
    }
}

pub fn can_book(existing: &[Booking], start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    for booking in existing {
        if start <= booking.end && end >= booking.start {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{can_book, Booking};

    #[test]
    fn allows_gap_between_existing_bookings() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, 21, 29));
    }

    #[test]
    fn allows_back_to_back_when_request_starts_at_previous_end() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, 20, 25));
    }

    #[test]
    fn allows_back_to_back_when_request_ends_at_next_start() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, 5, 10));
    }

    #[test]
    fn rejects_true_overlap_inside_active_booking() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, 12, 18));
    }

    #[test]
    fn rejects_request_that_spans_multiple_active_bookings() {
        let existing = [Booking::new(10, 15), Booking::new(20, 25)];
        assert!(!can_book(&existing, 14, 21));
    }

    #[test]
    fn ignores_cancelled_bookings_for_conflicts() {
        let existing = [Booking::cancelled(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, 12, 18));
        assert!(!can_book(&existing, 35, 36));
    }

    #[test]
    fn rejects_zero_length_requests() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, 25, 25));
    }

    #[test]
    fn rejects_reversed_ranges() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, 30, 25));
    }
}
