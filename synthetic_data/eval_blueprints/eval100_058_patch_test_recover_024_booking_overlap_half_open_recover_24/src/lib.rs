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
        if candidate.start <= b.end && b.start <= candidate.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{can_book, Booking};

    #[test]
    fn rejects_inverted_candidate() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(9, 3)));
    }

    #[test]
    fn allows_touching_boundaries_for_half_open_intervals() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(40, 45)));
        assert!(can_book(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn rejects_true_overlap_inside_existing() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
    }

    #[test]
    fn rejects_overlap_across_multiple_bookings() {
        let existing = [Booking::new(10, 20), Booking::new(25, 35)];
        assert!(!can_book(&existing, Booking::new(19, 26)));
    }

    #[test]
    fn rejects_zero_length_candidate() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(22, 22)));
    }

    #[test]
    fn empty_schedule_still_requires_positive_length() {
        let existing: [Booking; 0] = [];
        assert!(can_book(&existing, Booking::new(3, 8)));
        assert!(!can_book(&existing, Booking::new(7, 7)));
    }
}
