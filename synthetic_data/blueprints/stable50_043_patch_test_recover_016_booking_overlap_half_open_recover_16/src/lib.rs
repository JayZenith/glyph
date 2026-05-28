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

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    for b in existing {
        let overlap = candidate.start >= b.start && candidate.start < b.end
            || candidate.end > b.start && candidate.end <= b.end;
        if overlap {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
    }

    #[test]
    fn rejects_partial_overlap_at_start() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 12)));
    }

    #[test]
    fn rejects_partial_overlap_at_end() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(18, 25)));
    }

    #[test]
    fn rejects_when_candidate_contains_existing() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn rejects_zero_length_booking() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(20, 20)));
    }

    #[test]
    fn allows_booking_before_all_existing() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(1, 5)));
    }
}
