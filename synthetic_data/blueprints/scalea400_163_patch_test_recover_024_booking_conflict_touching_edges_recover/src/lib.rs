pub fn can_book(existing: &[(u32, u32)], new_slot: (u32, u32)) -> bool {
    let (start, end) = new_slot;
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
    fn rejects_invalid_slot() {
        assert!(!can_book(&[], (8, 8)));
        assert!(!can_book(&[], (9, 7)));
    }

    #[test]
    fn accepts_non_overlapping_gap() {
        let existing = [(10, 12), (14, 16)];
        assert!(can_book(&existing, (12, 14)));
    }

    #[test]
    fn accepts_touching_edges() {
        let existing = [(10, 12)];
        assert!(can_book(&existing, (12, 15)));
        assert!(can_book(&existing, (7, 10)));
    }

    #[test]
    fn rejects_overlap_at_start() {
        let existing = [(10, 12)];
        assert!(!can_book(&existing, (11, 13)));
    }

    #[test]
    fn rejects_containing_existing_slot() {
        let existing = [(10, 12)];
        assert!(!can_book(&existing, (9, 13)));
    }
}
