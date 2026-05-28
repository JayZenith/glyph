pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;

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
    fn allows_back_to_back_bookings() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 50)));
    }

    #[test]
    fn rejects_true_overlap_and_containment() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 35)));
        assert!(!can_book(&existing, (5, 45)));
    }

    #[test]
    fn rejects_invalid_request_ranges() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (12, 12)));
        assert!(!can_book(&existing, (25, 24)));
    }
}
