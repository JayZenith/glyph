#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Self { start, end })
        }
    }

    pub fn overlaps(&self, other: &Booking) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    for slot in existing {
        if slot.overlaps(&candidate) {
            return false;
        }
    }
    true
}

pub fn conflicting_bookings(existing: &[Booking], candidate: Booking) -> Vec<Booking> {
    existing
        .iter()
        .copied()
        .filter(|slot| slot.overlaps(&candidate))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn touching_endpoints_do_not_overlap() {
        let a = b(10, 12);
        let b2 = b(12, 15);
        assert!(!a.overlaps(&b2));
    }

    #[test]
    fn interior_overlap_is_detected() {
        let a = b(10, 14);
        let b2 = b(13, 18);
        assert!(a.overlaps(&b2));
    }

    #[test]
    fn contained_booking_conflicts() {
        let existing = [b(8, 20)];
        assert!(!can_book(&existing, b(10, 12)));
    }

    #[test]
    fn zero_length_booking_is_invalid() {
        assert_eq!(Booking::new(7, 7), None);
        assert!(!can_book(&[], Booking { start: 7, end: 7 }));
    }

    #[test]
    fn collects_and_sorts_conflicts_by_start_then_end() {
        let existing = [b(20, 25), b(10, 14), b(12, 13), b(30, 40)];
        let conflicts = conflicting_bookings(&existing, b(11, 21));
        assert_eq!(conflicts, vec![b(10, 14), b(12, 13), b(20, 25)]);
    }
}
