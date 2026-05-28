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

pub fn can_book(existing: &[Booking], request: Booking) -> bool {
    for slot in existing {
        if overlaps(slot, &request) {
            return false;
        }
    }
    true
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn adjacent_bookings_do_not_conflict() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(can_book(&existing, b(20, 30)));
        assert!(can_book(&existing, b(0, 10)));
        assert!(can_book(&existing, b(40, 50)));
    }

    #[test]
    fn partial_overlap_conflicts() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, b(5, 11)));
        assert!(!can_book(&existing, b(19, 25)));
        assert!(!can_book(&existing, b(10, 20)));
    }

    #[test]
    fn containment_conflicts_in_both_directions() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, b(12, 18)));
        assert!(!can_book(&existing, b(5, 25)));
    }

    #[test]
    fn multiple_existing_slots_only_allow_true_gaps() {
        let existing = [b(5, 10), b(15, 18), b(22, 30)];
        assert!(can_book(&existing, b(10, 15)));
        assert!(can_book(&existing, b(18, 22)));
        assert!(!can_book(&existing, b(9, 16)));
        assert!(!can_book(&existing, b(17, 23)));
    }

    #[test]
    fn invalid_booking_requests_are_rejected() {
        assert!(Booking::new(7, 7).is_none());
        assert!(Booking::new(9, 3).is_none());
    }
}
