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
    if candidate.start > candidate.end {
        return false;
    }
    !existing.iter().copied().any(|slot| overlaps(slot, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_endpoints_do_not_overlap() {
        let a = Booking::new(10, 20);
        let b = Booking::new(20, 30);
        assert!(!overlaps(a, b));
    }

    #[test]
    fn nested_ranges_overlap() {
        let a = Booking::new(10, 30);
        let b = Booking::new(15, 18);
        assert!(overlaps(a, b));
    }

    #[test]
    fn candidate_fitting_between_existing_is_allowed() {
        let existing = [Booking::new(0, 10), Booking::new(20, 30)];
        assert!(can_book(&existing, Booking::new(10, 20)));
    }

    #[test]
    fn zero_length_booking_is_rejected() {
        let existing = [Booking::new(5, 8)];
        assert!(!can_book(&existing, Booking::new(12, 12)));
    }
}
