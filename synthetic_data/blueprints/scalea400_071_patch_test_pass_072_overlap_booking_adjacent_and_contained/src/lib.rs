pub fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    let (start, end) = candidate;
    if start >= end {
        return false;
    }

    for &(s, e) in existing {
        if start >= s && start <= e {
            return false;
        }
        if end >= s && end <= e {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn rejects_overlap_in_middle() {
        assert!(!can_book(&[(10, 20)], (15, 18)));
    }

    #[test]
    fn allows_adjacent_before() {
        assert!(can_book(&[(10, 20)], (5, 10)));
    }

    #[test]
    fn allows_adjacent_after() {
        assert!(can_book(&[(10, 20)], (20, 25)));
    }

    #[test]
    fn rejects_covering_existing_slot() {
        assert!(!can_book(&[(10, 20)], (5, 25)));
    }

    #[test]
    fn rejects_invalid_zero_length_booking() {
        assert!(!can_book(&[(8, 12)], (12, 12)));
    }

    #[test]
    fn checks_against_all_existing_slots() {
        let existing = [(1, 3), (6, 9), (12, 14)];
        assert!(can_book(&existing, (3, 6)));
        assert!(!can_book(&existing, (8, 13)));
    }
}
