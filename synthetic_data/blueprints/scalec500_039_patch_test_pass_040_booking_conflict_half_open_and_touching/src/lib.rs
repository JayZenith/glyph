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

    for slot in existing {
        if candidate.start <= slot.end && slot.start <= candidate.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_reversed_or_empty_candidate() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(9, 9)));
        assert!(!can_book(&existing, Booking::new(12, 11)));
    }

    #[test]
    fn allows_touching_edges_for_half_open_intervals() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
        assert!(can_book(&existing, Booking::new(40, 50)));
    }

    #[test]
    fn rejects_partial_overlap_on_left_or_right() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 11)));
        assert!(!can_book(&existing, Booking::new(19, 25)));
    }

    #[test]
    fn rejects_containment_in_both_directions() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(8, 22)));
    }

    #[test]
    fn checks_against_all_existing_bookings() {
        let existing = [
            Booking::new(5, 8),
            Booking::new(10, 15),
            Booking::new(20, 25),
        ];
        assert!(!can_book(&existing, Booking::new(14, 21)));
        assert!(can_book(&existing, Booking::new(15, 20)));
    }
}
