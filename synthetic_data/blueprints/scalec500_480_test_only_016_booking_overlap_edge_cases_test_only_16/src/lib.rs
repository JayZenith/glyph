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
}

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start < b.end && b.start < a.end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|&slot| !overlaps(slot, candidate))
}

pub fn conflicting_bookings(existing: &[Booking], candidate: Booking) -> Vec<Booking> {
    let mut out: Vec<Booking> = existing
        .iter()
        .copied()
        .filter(|&slot| overlaps(slot, candidate))
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
    fn rejects_zero_length_or_reversed_ranges() {
        assert_eq!(Booking::new(5, 5), None);
        assert_eq!(Booking::new(8, 3), None);
        assert_eq!(Booking::new(0, 1), Some(Booking { start: 0, end: 1 }));
    }

    #[test]
    fn touching_intervals_do_not_overlap() {
        assert!(!overlaps(b(10, 20), b(20, 30)));
        assert!(!overlaps(b(0, 5), b(5, 9)));
    }

    #[test]
    fn partial_and_contained_intervals_do_overlap() {
        assert!(overlaps(b(10, 20), b(15, 25)));
        assert!(overlaps(b(10, 20), b(11, 19)));
        assert!(overlaps(b(11, 19), b(10, 20)));
    }

    #[test]
    fn can_book_requires_no_conflict_with_any_existing_slot() {
        let existing = [b(9, 11), b(13, 15), b(18, 21)];
        assert!(can_book(&existing, b(11, 13)));
        assert!(can_book(&existing, b(15, 18)));
        assert!(!can_book(&existing, b(10, 14)));
        assert!(!can_book(&existing, b(20, 22)));
    }

    #[test]
    fn conflicting_bookings_returns_sorted_conflicts_only() {
        let existing = [b(30, 40), b(10, 20), b(18, 22), b(50, 60)];
        let conflicts = conflicting_bookings(&existing, b(19, 35));
        assert_eq!(conflicts, vec![b(10, 20), b(18, 22), b(30, 40)]);
    }

    #[test]
    fn candidate_inside_gap_has_no_conflicts() {
        let existing = [b(1, 3), b(5, 7), b(9, 12)];
        assert!(conflicting_bookings(&existing, b(3, 5)).is_empty());
        assert!(can_book(&existing, b(7, 9)));
    }
}
