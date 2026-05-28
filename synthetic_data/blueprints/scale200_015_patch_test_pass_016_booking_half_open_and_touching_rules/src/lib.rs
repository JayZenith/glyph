#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    for b in existing {
        if b.room != room {
            continue;
        }

        let overlaps = start <= b.end && end >= b.start;
        if overlaps {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Booking> {
        vec![
            Booking { room: "A", start: 10, end: 20, canceled: false },
            Booking { room: "A", start: 25, end: 30, canceled: true },
            Booking { room: "B", start: 10, end: 20, canceled: false },
        ]
    }

    #[test]
    fn rejects_invalid_or_zero_length_requests() {
        let bookings = sample();
        assert!(!can_book(&bookings, "A", 15, 15));
        assert!(!can_book(&bookings, "A", 21, 20));
    }

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let bookings = sample();
        assert!(can_book(&bookings, "A", 20, 25));
        assert!(can_book(&bookings, "A", 5, 10));
    }

    #[test]
    fn canceled_bookings_are_ignored() {
        let bookings = sample();
        assert!(can_book(&bookings, "A", 25, 30));
        assert!(can_book(&bookings, "A", 24, 25));
    }

    #[test]
    fn detects_partial_and_contained_overlaps() {
        let bookings = sample();
        assert!(!can_book(&bookings, "A", 12, 18));
        assert!(!can_book(&bookings, "A", 5, 12));
        assert!(!can_book(&bookings, "A", 18, 22));
        assert!(!can_book(&bookings, "A", 5, 25));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let bookings = sample();
        assert!(can_book(&bookings, "C", 12, 18));
        assert!(!can_book(&bookings, "B", 12, 18));
    }
}
