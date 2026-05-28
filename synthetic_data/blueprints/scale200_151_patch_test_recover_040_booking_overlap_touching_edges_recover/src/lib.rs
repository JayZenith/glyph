pub fn can_book(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    if start >= end {
        return false;
    }

    for &(a, b) in existing {
        if start >= a && start < b {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn rejects_zero_length() {
        assert!(!can_book(&[], 5, 5));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, 20, 30));
        assert!(can_book(&existing, 0, 10));
        assert!(can_book(&existing, 40, 50));
    }

    #[test]
    fn rejects_overlap_when_new_start_is_inside() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, 15, 25));
    }

    #[test]
    fn rejects_overlap_when_new_contains_existing() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, 5, 25));
    }

    #[test]
    fn rejects_overlap_when_new_end_is_inside() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, 5, 15));
    }
}
