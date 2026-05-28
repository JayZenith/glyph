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
    fn rejects_invalid_interval() {
        assert!(!can_book(&[], 5, 5));
        assert!(!can_book(&[], 7, 3));
    }

    #[test]
    fn rejects_true_overlap() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, 15, 18));
        assert!(!can_book(&existing, 18, 22));
        assert!(!can_book(&existing, 35, 45));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, 20, 30));
        assert!(can_book(&existing, 0, 10));
        assert!(can_book(&existing, 40, 50));
    }
}
