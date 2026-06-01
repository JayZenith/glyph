#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start < end, "booking must have positive duration");
        Self { start, end }
    }
}

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|slot| {
        candidate.start <= slot.end && slot.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_conflict() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!has_conflict(&existing, Booking::new(20, 30)));
        assert!(!has_conflict(&existing, Booking::new(0, 10)));
        assert!(!has_conflict(&existing, Booking::new(40, 45)));
    }

    #[test]
    fn interior_overlap_and_containment_conflict() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(has_conflict(&existing, Booking::new(15, 18)));
        assert!(has_conflict(&existing, Booking::new(18, 22)));
        assert!(has_conflict(&existing, Booking::new(5, 12)));
        assert!(has_conflict(&existing, Booking::new(8, 35)));
    }

    #[test]
    fn gap_between_bookings_is_allowed() {
        let existing = [Booking::new(10, 20), Booking::new(25, 35)];
        assert!(!has_conflict(&existing, Booking::new(20, 25)));
        assert!(!has_conflict(&existing, Booking::new(0, 5)));
    }
}
