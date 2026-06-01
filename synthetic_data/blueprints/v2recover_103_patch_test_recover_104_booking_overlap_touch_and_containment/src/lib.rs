pub fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    let (start, end) = candidate;
    if start >= end {
        return false;
    }

    for &(a, b) in existing {
        if start <= b && end >= a {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn touching_edges_are_allowed() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 50)));
        assert!(can_book(&existing, (0, 10)));
    }

    #[test]
    fn overlaps_and_containment_are_rejected() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (5, 12)));
        assert!(!can_book(&existing, (18, 35)));
        assert!(!can_book(&existing, (8, 22)));
    }

    #[test]
    fn invalid_empty_or_reversed_booking_is_rejected() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, (7, 7)));
        assert!(!can_book(&existing, (9, 3)));
    }
}
