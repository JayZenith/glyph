pub fn can_book(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    if start >= end {
        return true;
    }

    for &(s, e) in existing {
        if start <= e && s <= end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn rejects_true_overlap() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, 15, 18));
        assert!(!can_book(&existing, 18, 35));
    }

    #[test]
    fn allows_touching_edges_for_half_open_intervals() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, 20, 30));
        assert!(can_book(&existing, 0, 10));
        assert!(can_book(&existing, 40, 50));
    }

    #[test]
    fn rejects_invalid_or_empty_requests() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, 12, 12));
        assert!(!can_book(&existing, 25, 20));
    }
}
