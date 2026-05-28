#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start < end);
        Self { start, end }
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|slot| {
        candidate.end < slot.start || candidate.start > slot.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_booking_inside_existing_slot() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
    }

    #[test]
    fn allows_non_overlapping_booking_before_slot() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(1, 5)));
    }

    #[test]
    fn allows_adjacent_booking_at_end_boundary() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 25)));
    }

    #[test]
    fn rejects_candidate_that_contains_existing_slot() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 25)));
    }
}
