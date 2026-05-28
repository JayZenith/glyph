#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            canceled: false,
        }
    }

    pub fn canceled(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            canceled: true,
        }
    }
}

pub fn can_book(existing: &[Booking], requested: &Booking) -> bool {
    if requested.canceled {
        return false;
    }

    for booking in existing {
        if booking.canceled {
            continue;
        }

        if overlaps(booking, requested) {
            return false;
        }
    }

    true
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_adjacent_bookings() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, &Booking::new(20, 25)));
        assert!(can_book(&existing, &Booking::new(5, 10)));
    }

    #[test]
    fn rejects_true_overlap() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, &Booking::new(19, 21)));
        assert!(!can_book(&existing, &Booking::new(9, 11)));
        assert!(!can_book(&existing, &Booking::new(10, 20)));
    }

    #[test]
    fn ignores_canceled_existing_bookings() {
        let existing = [Booking::canceled(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, &Booking::new(15, 18)));
        assert!(!can_book(&existing, &Booking::new(35, 36)));
    }

    #[test]
    fn rejects_invalid_or_canceled_requested_booking() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, &Booking::new(8, 8)));
        assert!(!can_book(&existing, &Booking::new(22, 21)));
        assert!(!can_book(&existing, &Booking::canceled(21, 25)));
    }

    #[test]
    fn rejects_when_request_contains_existing_or_is_contained() {
        let existing = [Booking::new(10, 20), Booking::new(30, 35)];
        assert!(!can_book(&existing, &Booking::new(5, 25)));
        assert!(!can_book(&existing, &Booking::new(31, 34)));
    }
}
