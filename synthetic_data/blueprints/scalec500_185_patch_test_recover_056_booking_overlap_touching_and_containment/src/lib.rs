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

pub fn conflicts(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| {
        candidate.start >= b.start && candidate.start < b.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(!conflicts(&existing, Booking::new(20, 30)));
        assert!(!conflicts(&existing, Booking::new(0, 10)));
    }

    #[test]
    fn overlapping_start_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(15, 25)));
    }

    #[test]
    fn candidate_containing_existing_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn gap_between_bookings_is_allowed() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!conflicts(&existing, Booking::new(20, 30)));
    }
}
