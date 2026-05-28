pub fn can_book(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
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
    fn rejects_overlap_inside_existing() {
        let bookings = [(10, 20)];
        assert!(!can_book(&bookings, 12, 18));
    }

    #[test]
    fn rejects_overlap_across_boundary() {
        let bookings = [(10, 20)];
        assert!(!can_book(&bookings, 18, 25));
    }

    #[test]
    fn allows_touching_endpoints() {
        let bookings = [(10, 20), (30, 40)];
        assert!(can_book(&bookings, 20, 30));
    }

    #[test]
    fn rejects_zero_length_booking() {
        assert!(!can_book(&[], 5, 5));
    }
}
