pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;
    if start > end {
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
    fn rejects_actual_overlap() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 32)));
    }

    #[test]
    fn allows_touching_edges() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 45)));
        assert!(can_book(&existing, (5, 10)));
    }

    #[test]
    fn rejects_invalid_or_contained() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, (25, 24)));
        assert!(!can_book(&existing, (12, 19)));
    }
}
