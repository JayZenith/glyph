pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    if request.0 >= request.1 {
        return false;
    }

    for &(start, end) in existing {
        if request.0 <= end && start <= request.1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn allows_non_overlapping_gap() {
        let existing = [(10, 12), (14, 16)];
        assert!(can_book(&existing, (12, 14)));
    }

    #[test]
    fn rejects_true_overlap() {
        let existing = [(10, 12), (14, 16)];
        assert!(!can_book(&existing, (11, 15)));
    }

    #[test]
    fn rejects_contained_interval() {
        let existing = [(8, 20)];
        assert!(!can_book(&existing, (10, 12)));
    }

    #[test]
    fn rejects_invalid_request() {
        assert!(!can_book(&[], (5, 5)));
        assert!(!can_book(&[], (9, 3)));
    }
}
