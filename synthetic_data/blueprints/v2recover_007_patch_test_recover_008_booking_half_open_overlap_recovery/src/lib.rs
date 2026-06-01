pub fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    if a_start > a_end || b_start > b_end {
        return false;
    }
    a_start <= b_end && b_start <= a_end
}

pub fn can_book(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    for &(s, e) in existing {
        if overlaps(s, e, start, end) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_ranges_do_not_overlap() {
        assert!(!overlaps(10, 12, 12, 14));
        assert!(can_book(&[(9, 12)], 12, 15));
    }

    #[test]
    fn interior_overlap_blocks_booking() {
        assert!(overlaps(10, 15, 11, 12));
        assert!(!can_book(&[(10, 15)], 11, 12));
    }

    #[test]
    fn exact_match_blocks_booking() {
        assert!(overlaps(8, 10, 8, 10));
        assert!(!can_book(&[(8, 10)], 8, 10));
    }

    #[test]
    fn zero_length_booking_is_invalid() {
        assert!(!can_book(&[], 7, 7));
    }
}
