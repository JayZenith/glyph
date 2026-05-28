#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Result<Self, &'static str> {
        if start >= end {
            return Err("start must be before end");
        }
        Ok(Self { start, end })
    }
}

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start < b.end && b.start < a.end
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| !overlaps(*b, candidate))
}

pub fn conflicting_indices(existing: &[Booking], candidate: Booking) -> Vec<usize> {
    existing
        .iter()
        .enumerate()
        .filter_map(|(i, b)| overlaps(*b, candidate).then_some(i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn rejects_invalid_booking() {
        assert!(Booking::new(5, 5).is_err());
        assert!(Booking::new(8, 3).is_err());
    }

    #[test]
    fn touching_endpoints_do_not_overlap() {
        assert!(!overlaps(b(10, 20), b(20, 25)));
        assert!(!overlaps(b(20, 25), b(10, 20)));
    }

    #[test]
    fn contained_and_partial_ranges_overlap() {
        assert!(overlaps(b(10, 30), b(12, 18)));
        assert!(overlaps(b(10, 30), b(25, 40)));
        assert!(overlaps(b(25, 40), b(10, 30)));
    }

    #[test]
    fn can_book_when_candidate_fits_gaps_or_touches_boundary() {
        let existing = [b(9, 10), b(12, 14), b(20, 22)];
        assert!(can_book(&existing, b(10, 12)));
        assert!(can_book(&existing, b(14, 20)));
        assert!(!can_book(&existing, b(13, 21)));
    }

    #[test]
    fn returns_all_conflict_indices_in_order() {
        let existing = [b(1, 4), b(4, 6), b(5, 8), b(10, 12), b(11, 15)];
        assert_eq!(conflicting_indices(&existing, b(3, 11)), vec![0, 1, 2, 3]);
        assert_eq!(conflicting_indices(&existing, b(8, 10)), Vec::<usize>::new());
        assert_eq!(conflicting_indices(&existing, b(11, 12)), vec![3, 4]);
    }
}
