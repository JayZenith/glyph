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
    for slot in existing {
        if slot.start == candidate.start && slot.end == candidate.end {
            continue;
        }
        if overlaps(*slot, candidate) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_overlap() {
        let a = Booking::new(10, 12);
        let b = Booking::new(12, 15);
        assert!(!overlaps(a, b));
    }

    #[test]
    fn interior_ranges_overlap() {
        let a = Booking::new(10, 14);
        let b = Booking::new(12, 16);
        assert!(overlaps(a, b));
    }

    #[test]
    fn contained_range_overlaps() {
        let a = Booking::new(8, 20);
        let b = Booking::new(11, 13);
        assert!(overlaps(a, b));
    }

    #[test]
    fn booking_is_allowed_when_only_touching_previous_end() {
        let existing = [Booking::new(9, 11), Booking::new(14, 16)];
        assert!(can_book(&existing, Booking::new(11, 14)));
    }

    #[test]
    fn booking_is_rejected_when_it_conflicts() {
        let existing = [Booking::new(9, 11), Booking::new(14, 16)];
        assert!(!can_book(&existing, Booking::new(10, 12)));
    }

    #[test]
    fn exact_duplicate_still_conflicts() {
        let existing = [Booking::new(9, 11)];
        assert!(!can_book(&existing, Booking::new(9, 11)));
    }
}
