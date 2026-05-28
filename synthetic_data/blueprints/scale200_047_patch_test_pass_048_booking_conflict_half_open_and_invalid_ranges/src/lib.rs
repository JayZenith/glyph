#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return true;
    }

    !existing.iter().any(|b| {
        candidate.start <= b.end && b.start <= candidate.end
    })
}

pub fn insert_booking(existing: &mut Vec<Booking>, candidate: Booking) -> bool {
    if can_book(existing, candidate) {
        existing.push(candidate);
        existing.sort_by_key(|b| (b.start, b.end));
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_booking() {
        let existing = vec![Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
        assert!(can_book(&existing, Booking::new(40, 50)));
    }

    #[test]
    fn rejects_true_overlap_cases() {
        let existing = vec![Booking::new(10, 20), Booking::new(30, 40)];
        assert!(!can_book(&existing, Booking::new(15, 18)));
        assert!(!can_book(&existing, Booking::new(5, 11)));
        assert!(!can_book(&existing, Booking::new(19, 31)));
        assert!(!can_book(&existing, Booking::new(35, 45)));
    }

    #[test]
    fn rejects_invalid_or_empty_ranges() {
        let existing = vec![Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(7, 7)));
        assert!(!can_book(&existing, Booking::new(9, 8)));

        let mut bookings = existing.clone();
        assert!(!insert_booking(&mut bookings, Booking::new(20, 20)));
        assert_eq!(bookings, existing);
    }

    #[test]
    fn insert_preserves_sorted_non_overlapping_schedule() {
        let mut bookings = vec![Booking::new(30, 40), Booking::new(10, 20)];
        assert!(insert_booking(&mut bookings, Booking::new(20, 30)));
        assert_eq!(
            bookings,
            vec![Booking::new(10, 20), Booking::new(20, 30), Booking::new(30, 40)]
        );

        assert!(!insert_booking(&mut bookings, Booking::new(25, 35)));
        assert_eq!(
            bookings,
            vec![Booking::new(10, 20), Booking::new(20, 30), Booking::new(30, 40)]
        );
    }
}
