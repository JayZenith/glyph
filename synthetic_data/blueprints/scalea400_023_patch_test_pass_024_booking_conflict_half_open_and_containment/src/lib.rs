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

pub fn can_book(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    for booking in existing {
        let starts_inside = request.start >= booking.start && request.start <= booking.end;
        let ends_inside = request.end >= booking.start && request.end <= booking.end;
        if starts_inside || ends_inside {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = [Booking::new(10, 20), Booking::new(30, 40)];
        assert!(can_book(&existing, Booking::new(20, 30)));
        assert!(can_book(&existing, Booking::new(0, 10)));
        assert!(can_book(&existing, Booking::new(40, 50)));
    }

    #[test]
    fn rejects_any_actual_overlap() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 11)));
        assert!(!can_book(&existing, Booking::new(19, 25)));
        assert!(!can_book(&existing, Booking::new(12, 18)));
    }

    #[test]
    fn rejects_when_request_contains_existing_booking() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(5, 25)));
    }

    #[test]
    fn rejects_zero_length_or_inverted_requests() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(15, 15)));
        assert!(!can_book(&existing, Booking::new(21, 20)));
    }

    #[test]
    fn checks_against_multiple_existing_bookings() {
        let existing = [Booking::new(0, 5), Booking::new(10, 15), Booking::new(20, 25)];
        assert!(can_book(&existing, Booking::new(5, 10)));
        assert!(!can_book(&existing, Booking::new(14, 21)));
    }
}
