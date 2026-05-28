pub fn overlaps(existing: (u32, u32), requested: (u32, u32)) -> bool {
    let (a_start, a_end) = existing;
    let (b_start, b_end) = requested;

    a_start <= b_end && b_start <= a_end
}

pub fn can_book(existing: &[(u32, u32)], requested: (u32, u32)) -> bool {
    if requested.0 >= requested.1 {
        return false;
    }

    !existing.iter().any(|&slot| overlaps(slot, requested))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_overlapping_booking() {
        let existing = [(10, 12), (15, 18)];
        assert!(!can_book(&existing, (11, 13)));
    }

    #[test]
    fn allows_back_to_back_booking() {
        let existing = [(10, 12), (15, 18)];
        assert!(can_book(&existing, (12, 15)));
    }

    #[test]
    fn rejects_exact_same_slot() {
        let existing = [(10, 12)];
        assert!(!can_book(&existing, (10, 12)));
    }

    #[test]
    fn rejects_zero_length_request() {
        let existing = [(10, 12)];
        assert!(!can_book(&existing, (8, 8)));
    }
}
