#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            cancelled: false,
        }
    }
}

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    if requested.start >= requested.end {
        return false;
    }

    for b in existing {
        if b.cancelled {
            continue;
        }
        if overlaps(*b, requested) {
            return false;
        }
    }
    true
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_requested_range() {
        assert!(!can_book(&[], Booking::new(20, 20)));
        assert!(!can_book(&[], Booking::new(30, 10)));
    }

    #[test]
    fn allows_adjacent_bookings() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
    }

    #[test]
    fn rejects_true_overlap_and_containment() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(15, 25)));
        assert!(!can_book(&existing, Booking::new(5, 12)));
        assert!(!can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn ignores_cancelled_bookings_when_checking_conflicts() {
        let existing = [
            Booking::new(0, 5),
            Booking {
                start: 10,
                end: 20,
                cancelled: true,
            },
            Booking::new(30, 40),
        ];
        assert!(can_book(&existing, Booking::new(12, 18)));
        assert!(!can_book(&existing, Booking::new(32, 35)));
    }

    #[test]
    fn multiple_existing_bookings_still_allow_gaps() {
        let existing = [Booking::new(0, 10), Booking::new(15, 25), Booking::new(30, 35)];
        assert!(can_book(&existing, Booking::new(10, 15)));
        assert!(can_book(&existing, Booking::new(25, 30)));
        assert!(!can_book(&existing, Booking::new(9, 16)));
    }
}
