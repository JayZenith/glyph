pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;
    if start >= end {
        return false;
    }

    for &(a, b) in existing {
        if a >= b {
            continue;
        }
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
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 30)));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 50)));
        assert!(can_book(&existing, (0, 10)));
    }

    #[test]
    fn rejects_invalid_request() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (5, 5)));
        assert!(!can_book(&existing, (8, 7)));
    }

    #[test]
    fn ignores_invalid_existing_ranges() {
        let existing = [(10, 10), (25, 20)];
        assert!(can_book(&existing, (10, 20)));
    }
}
