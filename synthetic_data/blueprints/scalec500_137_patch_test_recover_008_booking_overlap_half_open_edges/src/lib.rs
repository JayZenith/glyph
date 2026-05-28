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

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|slot| {
        candidate.start <= slot.end && slot.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_overlapping_booking() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(has_conflict(&existing, Booking::new(15, 18)));
        assert!(has_conflict(&existing, Booking::new(18, 35)));
    }

    #[test]
    fn touching_endpoints_do_not_conflict_for_half_open_ranges() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!has_conflict(&existing, Booking::new(20, 30)));
        assert!(!has_conflict(&existing, Booking::new(0, 10)));
        assert!(!has_conflict(&existing, Booking::new(40, 45)));
    }

    #[test]
    fn zero_length_candidate_never_blocks_time() {
        let existing = [Booking::new(10, 20)];
        assert!(!has_conflict(&existing, Booking::new(12, 12)));
    }
}
