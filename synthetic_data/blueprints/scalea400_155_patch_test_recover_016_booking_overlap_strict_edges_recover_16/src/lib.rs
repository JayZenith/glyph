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

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    existing.iter().all(|b| !overlaps(*b, requested))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_ranges_can_book() {
        let existing = [Booking::new(2, 4), Booking::new(6, 8)];
        assert!(can_book(&existing, Booking::new(4, 6)));
    }

    #[test]
    fn inner_overlap_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 15)));
    }

    #[test]
    fn partial_overlap_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(18, 22)));
    }

    #[test]
    fn empty_existing_allows_booking() {
        let existing: [Booking; 0] = [];
        assert!(can_book(&existing, Booking::new(1, 3)));
    }
}
