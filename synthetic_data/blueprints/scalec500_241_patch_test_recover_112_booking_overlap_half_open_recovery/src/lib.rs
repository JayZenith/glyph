pub fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    let (start, end) = candidate;
    if start > end {
        return false;
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
    fn rejects_overlapping_booking() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 33)));
    }

    #[test]
    fn allows_back_to_back_half_open_booking() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (0, 10)));
        assert!(can_book(&existing, (40, 45)));
    }

    #[test]
    fn rejects_invalid_or_zero_length_candidate() {
        let existing = vec![(10, 20)];
        assert!(!can_book(&existing, (8, 8)));
        assert!(!can_book(&existing, (9, 7)));
    }
}
