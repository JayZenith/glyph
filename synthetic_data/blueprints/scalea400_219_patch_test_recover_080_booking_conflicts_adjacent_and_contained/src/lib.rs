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

pub fn has_conflict(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return true;
    }

    existing.iter().any(|slot| {
        request.start >= slot.start && request.start <= slot.end
            || request.end >= slot.start && request.end <= slot.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_length_request() {
        let existing = [Booking::new(10, 12)];
        assert!(has_conflict(&existing, Booking::new(8, 8)));
    }

    #[test]
    fn allows_adjacent_request_at_end_boundary() {
        let existing = [Booking::new(10, 12)];
        assert!(!has_conflict(&existing, Booking::new(12, 14)));
    }

    #[test]
    fn detects_request_fully_contained_in_existing_slot() {
        let existing = [Booking::new(10, 20)];
        assert!(has_conflict(&existing, Booking::new(12, 14)));
    }

    #[test]
    fn allows_gap_between_multiple_bookings() {
        let existing = [Booking::new(2, 4), Booking::new(6, 9)];
        assert!(!has_conflict(&existing, Booking::new(4, 6)));
    }

    #[test]
    fn detects_overlap_across_left_edge() {
        let existing = [Booking::new(10, 15)];
        assert!(has_conflict(&existing, Booking::new(8, 11)));
    }
}
