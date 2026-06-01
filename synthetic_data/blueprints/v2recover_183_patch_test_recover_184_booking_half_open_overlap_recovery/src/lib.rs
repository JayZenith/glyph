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

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start >= candidate.end {
        return true;
    }

    for slot in existing {
        if candidate.start <= slot.end && slot.start <= candidate.end {
            return false;
        }
    }

    true
}

pub fn conflicts(existing: &[Booking], candidate: Booking) -> Vec<Booking> {
    existing
        .iter()
        .copied()
        .filter(|slot| candidate.start <= slot.end && slot.start <= candidate.end)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end)
    }

    #[test]
    fn touching_endpoints_do_not_overlap() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(can_book(&existing, b(20, 30)));
        assert!(conflicts(&existing, b(20, 30)).is_empty());
    }

    #[test]
    fn overlapping_inside_existing_is_rejected() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, b(12, 18)));
        assert_eq!(conflicts(&existing, b(12, 18)), vec![b(10, 20)]);
    }

    #[test]
    fn invalid_candidate_is_rejected_and_has_no_conflicts() {
        let existing = [b(5, 10)];
        assert!(!can_book(&existing, b(8, 8)));
        assert!(!can_book(&existing, b(9, 7)));
        assert!(conflicts(&existing, b(8, 8)).is_empty());
        assert!(conflicts(&existing, b(9, 7)).is_empty());
    }

    #[test]
    fn reports_all_conflicts_in_original_order() {
        let existing = [b(0, 5), b(5, 10), b(8, 12), b(15, 20)];
        assert_eq!(conflicts(&existing, b(4, 9)), vec![b(0, 5), b(5, 10), b(8, 12)]);
    }
}
