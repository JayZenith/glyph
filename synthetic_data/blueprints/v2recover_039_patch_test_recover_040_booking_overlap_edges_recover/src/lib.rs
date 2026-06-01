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
    if candidate.start >= candidate.end {
        return false;
    }

    for slot in existing {
        if candidate.start <= slot.end && slot.start <= candidate.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_endpoints_are_allowed() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
        assert!(can_book(&existing, Booking::new(40, 45)));
    }

    #[test]
    fn overlapping_ranges_are_rejected() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!can_book(&existing, Booking::new(15, 18)));
        assert!(!can_book(&existing, Booking::new(18, 22)));
        assert!(!can_book(&existing, Booking::new(35, 50)));
    }

    #[test]
    fn invalid_or_containing_ranges_are_rejected() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!can_book(&existing, Booking::new(25, 25)));
        assert!(!can_book(&existing, Booking::new(0, 50)));
    }
}
