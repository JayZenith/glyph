pub fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    let (start, end) = candidate;
    if start >= end {
        return false;
    }

    existing.iter().all(|&(s, e)| end < s || start > e)
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn allows_when_touching_endpoints_only() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (0, 10)));
        assert!(can_book(&existing, (40, 50)));
    }

    #[test]
    fn rejects_partial_overlap() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 25)));
        assert!(!can_book(&existing, (25, 35)));
    }

    #[test]
    fn rejects_contained_and_containing_ranges() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, (12, 18)));
        assert!(!can_book(&existing, (5, 25)));
    }

    #[test]
    fn rejects_invalid_candidate_range() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, (7, 7)));
        assert!(!can_book(&existing, (9, 3)));
    }
}
