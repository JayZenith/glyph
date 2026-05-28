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

pub fn conflicts(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| {
        candidate.start <= b.end && b.start <= candidate.end
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
    fn overlapping_ranges_conflict() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(15, 25)));
        assert!(conflicts(&existing, Booking::new(5, 11)));
    }

    #[test]
    fn contained_range_conflicts() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(12, 18)));
        assert!(conflicts(&existing, Booking::new(8, 22)));
    }
}
