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

    for booking in existing {
        if overlaps(*booking, candidate) {
            return false;
        }
    }
    true
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_inside_existing_is_rejected() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(12, 18)));
    }

    #[test]
    fn touching_endpoints_do_not_conflict_for_half_open_ranges() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 25)));
        assert!(can_book(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn invalid_or_empty_candidate_is_rejected() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(9, 9)));
        assert!(!can_book(&existing, Booking::new(21, 20)));
    }

    #[test]
    fn multiple_existing_bookings_are_checked() {
        let existing = [Booking::new(0, 5), Booking::new(8, 10), Booking::new(12, 15)];
        assert!(can_book(&existing, Booking::new(5, 8)));
        assert!(!can_book(&existing, Booking::new(9, 13)));
    }
}
