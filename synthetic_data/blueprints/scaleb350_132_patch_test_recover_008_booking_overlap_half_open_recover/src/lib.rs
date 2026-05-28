pub fn can_book(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    if start >= end {
        return false;
    }

    for &(a, b) in existing {
        if start <= b && a <= end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn rejects_real_overlap() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, 15, 18));
        assert!(!can_book(&existing, 18, 35));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, 20, 30));
        assert!(can_book(&existing, 0, 10));
        assert!(can_book(&existing, 40, 50));
    }

    #[test]
    fn rejects_invalid_or_contained_ranges() {
        let existing = [(5, 8)];
        assert!(!can_book(&existing, 6, 7));
        assert!(!can_book(&existing, 9, 9));
        assert!(!can_book(&existing, 12, 11));
    }
}
