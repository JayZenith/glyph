pub fn can_book(existing: &[(u32, u32)], new_slot: (u32, u32)) -> bool {
    let (start, end) = new_slot;
    if start >= end {
        return false;
    }

    for &(s, e) in existing {
        if s <= end && start <= e {
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
        assert!(!can_book(&[], (5, 5)));
        assert!(!can_book(&[], (8, 3)));
    }

    #[test]
    fn rejects_true_overlap() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 22)));
        assert!(!can_book(&existing, (35, 45)));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (0, 10)));
        assert!(can_book(&existing, (40, 50)));
    }

    #[test]
    fn rejects_containing_existing_slot() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (5, 25)));
    }
}
