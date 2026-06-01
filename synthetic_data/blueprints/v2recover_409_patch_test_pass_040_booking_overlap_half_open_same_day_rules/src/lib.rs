#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
        }
    }
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    for booking in existing {
        if booking.room != room {
            continue;
        }

        if start <= booking.end && booking.start <= end {
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
            Booking::new("A", 10, 20),
            Booking::new("A", 25, 30),
            Booking::new("B", 10, 15),
        ]
    }

    #[test]
    fn allows_back_to_back_in_same_room() {
        let bookings = sample();
        assert!(can_book(&bookings, "A", 20, 25));
        assert!(can_book(&bookings, "A", 30, 35));
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let bookings = sample();
        assert!(!can_book(&bookings, "A", 19, 26));
        assert!(!can_book(&bookings, "A", 12, 18));
        assert!(!can_book(&bookings, "A", 24, 26));
    }

    #[test]
    fn ignores_other_rooms() {
        let bookings = sample();
        assert!(can_book(&bookings, "B", 15, 20));
        assert!(can_book(&bookings, "C", 10, 20));
    }

    #[test]
    fn rejects_invalid_or_duplicate_ranges() {
        let bookings = sample();
        assert!(!can_book(&bookings, "A", 22, 22));
        assert!(!can_book(&bookings, "A", 30, 29));
        assert!(!can_book(&bookings, "A", 10, 20));
    }

    #[test]
    fn containment_is_conflict() {
        let bookings = sample();
        assert!(!can_book(&bookings, "A", 11, 19));
        assert!(!can_book(&bookings, "A", 9, 21));
    }
}
