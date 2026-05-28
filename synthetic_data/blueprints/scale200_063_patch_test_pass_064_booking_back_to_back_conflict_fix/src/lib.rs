#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Option<Self> {
        if start >= end {
            None
        } else {
            Some(Self { start, end })
        }
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| candidate.end < b.start || candidate.start > b.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn allows_booking_when_schedule_is_empty() {
        assert!(can_book(&[], b(10, 12)));
    }

    #[test]
    fn rejects_actual_overlap_inside_existing() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, b(12, 15)));
    }

    #[test]
    fn rejects_overlap_spanning_existing_boundary() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, b(5, 12)));
        assert!(!can_book(&existing, b(18, 25)));
    }

    #[test]
    fn allows_back_to_back_before_and_after() {
        let existing = [b(10, 20)];
        assert!(can_book(&existing, b(5, 10)));
        assert!(can_book(&existing, b(20, 25)));
    }

    #[test]
    fn checks_against_multiple_existing_bookings() {
        let existing = [b(8, 10), b(12, 14), b(20, 22)];
        assert!(can_book(&existing, b(10, 12)));
        assert!(!can_book(&existing, b(9, 13)));
        assert!(!can_book(&existing, b(14, 21)));
    }

    #[test]
    fn invalid_booking_is_rejected_by_constructor() {
        assert_eq!(Booking::new(7, 7), None);
        assert_eq!(Booking::new(9, 3), None);
    }
}
