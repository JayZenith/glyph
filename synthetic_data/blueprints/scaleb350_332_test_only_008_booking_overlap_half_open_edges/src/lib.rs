pub fn can_book(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    if request.0 >= request.1 {
        return false;
    }

    for &(start, end) in existing {
        if start >= end {
            continue;
        }
        if overlaps((start, end), request) {
            return false;
        }
    }
    true
}

fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

#[cfg(test)]
mod tests {
    use super::can_book;

    #[test]
    fn allows_adjacent_bookings() {
        let existing = [(10, 20), (30, 35)];
        assert!(can_book(&existing, (20, 30)));
        assert!(can_book(&existing, (35, 40)));
    }

    #[test]
    fn rejects_partial_overlap_on_left_or_right() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (5, 11)));
        assert!(!can_book(&existing, (19, 25)));
    }

    #[test]
    fn rejects_contained_and_containing_intervals() {
        let existing = [(10, 20)];
        assert!(!can_book(&existing, (12, 18)));
        assert!(!can_book(&existing, (8, 22)));
    }

    #[test]
    fn ignores_invalid_existing_slots_but_rejects_invalid_request() {
        let existing = [(5, 5), (9, 7), (20, 30)];
        assert!(can_book(&existing, (10, 15)));
        assert!(!can_book(&existing, (15, 15)));
    }

    #[test]
    fn rejects_when_overlapping_any_of_multiple_bookings() {
        let existing = [(0, 4), (6, 10), (12, 16)];
        assert!(!can_book(&existing, (3, 7)));
        assert!(!can_book(&existing, (10, 13)));
        assert!(can_book(&existing, (4, 6)));
    }
}
