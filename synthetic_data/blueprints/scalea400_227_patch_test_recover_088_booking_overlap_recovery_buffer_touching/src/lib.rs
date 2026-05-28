#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
    pub cleanup: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32, cleanup: u32) -> Self {
        Self { start, end, cleanup }
    }
}

pub fn overlaps(existing: &Booking, candidate: &Booking) -> bool {
    if existing.start > existing.end || candidate.start > candidate.end {
        return false;
    }

    let existing_end = existing.end;
    let candidate_end = candidate.end;

    existing.start <= candidate_end && candidate.start <= existing_end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| !overlaps(b, &candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_without_cleanup_is_allowed() {
        let existing = [Booking::new(10, 20, 0)];
        assert!(can_book(&existing, Booking::new(20, 30, 0)));
    }

    #[test]
    fn cleanup_extends_the_blocked_window() {
        let existing = [Booking::new(10, 20, 5)];
        assert!(!can_book(&existing, Booking::new(24, 30, 0)));
        assert!(can_book(&existing, Booking::new(25, 30, 0)));
    }

    #[test]
    fn zero_length_intervals_are_invalid() {
        let existing = [Booking::new(10, 20, 0)];
        assert!(!can_book(&existing, Booking::new(15, 15, 0)));
        assert!(!can_book(&[Booking::new(8, 8, 0)], Booking::new(9, 10, 0)));
    }

    #[test]
    fn overlap_is_detected_in_either_direction() {
        let a = Booking::new(30, 40, 0);
        let b = Booking::new(35, 45, 0);
        assert!(overlaps(&a, &b));
        assert!(overlaps(&b, &a));
    }
}
