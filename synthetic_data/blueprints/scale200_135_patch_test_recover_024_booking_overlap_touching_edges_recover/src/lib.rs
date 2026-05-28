pub fn has_conflict(existing: &[(u32, u32)], requested: (u32, u32)) -> bool {
    existing.iter().any(|&(start, end)| requested.0 <= end && start <= requested.1)
}

pub fn can_book(existing: &[(u32, u32)], requested: (u32, u32)) -> bool {
    requested.0 < requested.1 && !has_conflict(existing, requested)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_booking_is_rejected() {
        let existing = [(10, 20), (30, 40)];
        assert!(!can_book(&existing, (15, 18)));
        assert!(!can_book(&existing, (18, 35)));
    }

    #[test]
    fn touching_edges_are_allowed() {
        let existing = [(10, 20), (30, 40)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (40, 45)));
        assert!(can_book(&existing, (5, 10)));
    }

    #[test]
    fn invalid_empty_booking_is_rejected() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (12, 12)));
    }
}
