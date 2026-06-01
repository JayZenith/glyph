pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;
    if start >= end {
        return false;
    }

    !existing.iter().any(|&(a, b)| start < b && a < end)
}

pub fn conflicting_slots(existing: &[(u32, u32)], request: (u32, u32)) -> Vec<(u32, u32)> {
    let (start, end) = request;
    if start >= end {
        return Vec::new();
    }

    existing
        .iter()
        .copied()
        .filter(|&(a, b)| start < b && a < end)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{can_book, conflicting_slots};

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = vec![(9, 11), (13, 15)];
        assert!(can_book(&existing, (11, 13)));
        assert!(conflicting_slots(&existing, (11, 13)).is_empty());
    }

    #[test]
    fn rejects_overlap_and_lists_conflicts_in_input_order() {
        let existing = vec![(9, 12), (14, 16), (10, 11), (16, 18)];
        assert!(!can_book(&existing, (10, 15)));
        assert_eq!(conflicting_slots(&existing, (10, 15)), vec![(9, 12), (14, 16), (10, 11)]);
    }

    #[test]
    fn containment_counts_as_conflict() {
        let existing = vec![(8, 17)];
        assert!(!can_book(&existing, (10, 12)));
        assert_eq!(conflicting_slots(&existing, (10, 12)), vec![(8, 17)]);
    }

    #[test]
    fn zero_length_request_is_invalid() {
        let existing = vec![(9, 10)];
        assert!(!can_book(&existing, (10, 10)));
        assert!(conflicting_slots(&existing, (10, 10)).is_empty());
    }
}
