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

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| {
        candidate.start <= b.end && candidate.end >= b.start
    })
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    !has_conflict(existing, candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn rejects_simple_overlap() {
        let existing = [b(10, 20)];
        assert!(has_conflict(&existing, b(15, 25)));
        assert!(!can_book(&existing, b(15, 25)));
    }

    #[test]
    fn allows_touching_edges_for_half_open_intervals() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(!has_conflict(&existing, b(20, 30)));
        assert!(can_book(&existing, b(20, 30)));
    }

    #[test]
    fn rejects_candidate_fully_inside_existing() {
        let existing = [b(10, 50)];
        assert!(has_conflict(&existing, b(20, 30)));
    }

    #[test]
    fn rejects_existing_fully_inside_candidate() {
        let existing = [b(20, 30)];
        assert!(has_conflict(&existing, b(10, 40)));
    }

    #[test]
    fn constructor_rejects_empty_or_reversed_ranges() {
        assert_eq!(Booking::new(5, 5), None);
        assert_eq!(Booking::new(9, 2), None);
    }
}
