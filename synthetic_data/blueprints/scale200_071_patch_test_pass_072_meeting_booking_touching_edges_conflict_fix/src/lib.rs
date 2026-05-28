#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start <= end, "start must be <= end");
        Self { start, end }
    }
}

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    existing.iter().all(|slot| {
        requested.end <= slot.start || requested.start >= slot.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_gap_before_existing() {
        let existing = [Booking::new(20, 30)];
        assert!(can_book(&existing, Booking::new(10, 15)));
    }

    #[test]
    fn allows_touching_endpoints() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(40, 45)));
        assert!(can_book(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn rejects_partial_overlap() {
        let existing = [Booking::new(10, 20), Booking::new(25, 35)];
        assert!(!can_book(&existing, Booking::new(18, 22)));
        assert!(!can_book(&existing, Booking::new(24, 26)));
    }

    #[test]
    fn rejects_contained_and_exact_match() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(10, 20)));
    }

    #[test]
    fn zero_length_request_only_allowed_on_free_boundary() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 20)));
        assert!(!can_book(&existing, Booking::new(15, 15)));
    }
}
