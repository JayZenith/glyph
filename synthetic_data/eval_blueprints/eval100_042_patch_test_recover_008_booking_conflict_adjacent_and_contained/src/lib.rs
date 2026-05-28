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
    existing.iter().any(|slot| {
        candidate.start <= slot.end && slot.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_partial_overlap() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(15, 25)));
    }

    #[test]
    fn allows_back_to_back_booking() {
        let existing = [Booking::new(10, 20)];
        assert!(!conflicts(&existing, Booking::new(20, 30)));
    }

    #[test]
    fn rejects_booking_fully_inside_existing() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(12, 18)));
    }

    #[test]
    fn rejects_booking_that_wraps_existing() {
        let existing = [Booking::new(10, 20)];
        assert!(conflicts(&existing, Booking::new(5, 25)));
    }
}
