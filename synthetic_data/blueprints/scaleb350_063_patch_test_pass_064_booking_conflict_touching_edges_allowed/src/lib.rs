#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start < end, "start must be before end");
        Self { start, end }
    }
}

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|&slot| !overlaps(slot, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let existing = [Booking::new(9, 11), Booking::new(13, 15)];
        assert!(can_book(&existing, Booking::new(11, 13)));
        assert!(can_book(&existing, Booking::new(15, 16)));
    }

    #[test]
    fn partial_overlap_conflicts() {
        let existing = [Booking::new(9, 12)];
        assert!(!can_book(&existing, Booking::new(11, 14)));
        assert!(!can_book(&existing, Booking::new(8, 10)));
    }

    #[test]
    fn contained_and_covering_intervals_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(8, 22)));
    }

    #[test]
    fn separate_intervals_are_allowed() {
        let existing = [Booking::new(2, 4), Booking::new(7, 9)];
        assert!(can_book(&existing, Booking::new(4, 7)));
        assert!(can_book(&existing, Booking::new(9, 12)));
        assert!(can_book(&existing, Booking::new(0, 2)));
    }
}
