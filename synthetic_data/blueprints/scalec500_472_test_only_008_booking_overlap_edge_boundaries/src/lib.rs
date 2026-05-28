#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Option<Self> {
        if start < end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    pub fn contains(self, minute: u32) -> bool {
        self.start <= minute && minute < self.end
    }
}

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    existing.iter().all(|b| !b.overlaps(requested))
}

pub fn conflicting_bookings(existing: &[Booking], requested: Booking) -> Vec<Booking> {
    let mut out: Vec<Booking> = existing
        .iter()
        .copied()
        .filter(|b| b.overlaps(requested))
        .collect();
    out.sort_by_key(|b| (b.start, b.end));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn rejects_zero_length_and_reversed_ranges() {
        assert_eq!(Booking::new(10, 10), None);
        assert_eq!(Booking::new(15, 14), None);
        assert_eq!(Booking::new(0, 1), Some(Booking { start: 0, end: 1 }));
    }

    #[test]
    fn touching_edges_do_not_overlap() {
        let a = b(60, 120);
        let c = b(120, 180);
        assert!(!a.overlaps(c));
        assert!(!c.overlaps(a));
    }

    #[test]
    fn partial_and_contained_ranges_do_overlap() {
        let base = b(100, 200);
        assert!(base.overlaps(b(150, 250)));
        assert!(base.overlaps(b(50, 150)));
        assert!(base.overlaps(b(120, 180)));
        assert!(b(120, 180).overlaps(base));
    }

    #[test]
    fn contains_uses_half_open_interval() {
        let slot = b(30, 45);
        assert!(slot.contains(30));
        assert!(slot.contains(44));
        assert!(!slot.contains(45));
        assert!(!slot.contains(29));
    }

    #[test]
    fn can_book_accepts_only_non_conflicting_request() {
        let existing = [b(0, 30), b(45, 60), b(90, 120)];
        assert!(can_book(&existing, b(30, 45)));
        assert!(can_book(&existing, b(60, 90)));
        assert!(!can_book(&existing, b(20, 50)));
        assert!(!can_book(&existing, b(119, 121)));
    }

    #[test]
    fn conflicting_bookings_returns_sorted_conflicts_only() {
        let existing = [b(50, 70), b(10, 20), b(30, 40), b(35, 55), b(70, 80)];
        let got = conflicting_bookings(&existing, b(18, 36));
        assert_eq!(got, vec![b(10, 20), b(30, 40), b(35, 55)]);
    }
}
