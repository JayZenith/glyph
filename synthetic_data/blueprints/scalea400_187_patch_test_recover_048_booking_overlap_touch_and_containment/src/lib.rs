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

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| candidate.start < b.end && candidate.end > b.start)
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    !existing.iter().any(|b| candidate.start <= b.end && candidate.end >= b.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_ranges_do_not_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 25)));
        assert!(can_book(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn overlapping_ranges_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(15, 18)));
        assert!(!can_book(&existing, Booking::new(18, 22)));
    }

    #[test]
    fn contained_existing_booking_still_blocks() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn helper_reports_conflict() {
        let existing = [Booking::new(30, 40), Booking::new(50, 60)];
        assert!(has_conflict(&existing, Booking::new(35, 38)));
        assert!(!has_conflict(&existing, Booking::new(40, 50)));
    }
}
