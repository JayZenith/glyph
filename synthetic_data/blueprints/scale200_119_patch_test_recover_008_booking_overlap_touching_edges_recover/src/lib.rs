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

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| !overlaps(*b, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_conflict() {
        let a = Booking::new(10, 20);
        let b = Booking::new(20, 30);
        assert!(!overlaps(a, b));
    }

    #[test]
    fn contained_ranges_conflict() {
        let a = Booking::new(10, 30);
        let b = Booking::new(15, 20);
        assert!(overlaps(a, b));
    }

    #[test]
    fn candidate_that_starts_inside_existing_is_rejected() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(15, 25)));
    }

    #[test]
    fn candidate_before_all_bookings_is_allowed() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(1, 5)));
    }

    #[test]
    fn candidate_spanning_multiple_bookings_is_rejected() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!can_book(&existing, Booking::new(5, 35)));
    }
}
