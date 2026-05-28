pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;
    if start >= end {
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
    fn rejects_overlapping_middle_segment() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 35)));
    }

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 50)));
        assert!(can_book(&existing, (0, 10)));
    }

    #[test]
    fn rejects_zero_length_request() {
        let existing = vec![(5, 10)];
        assert!(!can_book(&existing, (7, 7)));
    }
}
