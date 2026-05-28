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

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start >= candidate.end {
        return true;
    }
    !existing.iter().any(|&slot| overlaps(slot, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_overlap() {
        let a = Booking::new(10, 20);
        let b = Booking::new(20, 30);
        assert!(!overlaps(a, b));
    }

    #[test]
    fn contained_range_overlaps() {
        let a = Booking::new(10, 30);
        let b = Booking::new(15, 18);
        assert!(overlaps(a, b));
    }

    #[test]
    fn booking_is_rejected_when_it_hits_existing_slot() {
        let existing = [Booking::new(9, 12), Booking::new(15, 18)];
        assert!(!can_book(&existing, Booking::new(11, 16)));
    }

    #[test]
    fn booking_is_allowed_when_it_only_touches_boundaries() {
        let existing = [Booking::new(9, 12), Booking::new(15, 18)];
        assert!(can_book(&existing, Booking::new(12, 15)));
    }

    #[test]
    fn invalid_candidate_is_not_bookable() {
        let existing = [Booking::new(9, 12)];
        assert!(!can_book(&existing, Booking::new(7, 7)));
    }
}
