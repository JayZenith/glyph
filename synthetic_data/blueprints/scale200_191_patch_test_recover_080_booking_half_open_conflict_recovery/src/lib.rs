#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub room: u32,
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(room: u32, start: u32, end: u32) -> Self {
        Self { room, start, end }
    }
}

pub fn can_book(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    for b in existing {
        if b.room != request.room {
            continue;
        }

        if request.start <= b.end && b.start <= request.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_reversed_or_empty_requests() {
        let existing = [Booking::new(1, 10, 12)];
        assert!(!can_book(&existing, Booking::new(1, 8, 7)));
        assert!(!can_book(&existing, Booking::new(1, 9, 9)));
    }

    #[test]
    fn allows_adjacent_in_same_room() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(can_book(&existing, Booking::new(1, 20, 25)));
        assert!(can_book(&existing, Booking::new(1, 5, 10)));
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let existing = [
            Booking::new(1, 10, 20),
            Booking::new(1, 30, 35),
        ];
        assert!(!can_book(&existing, Booking::new(1, 19, 22)));
        assert!(!can_book(&existing, Booking::new(1, 34, 40)));
        assert!(!can_book(&existing, Booking::new(1, 12, 18)));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = [Booking::new(2, 10, 20)];
        assert!(can_book(&existing, Booking::new(1, 15, 18)));
    }

    #[test]
    fn bridges_gap_between_two_bookings_is_rejected() {
        let existing = [
            Booking::new(1, 10, 15),
            Booking::new(1, 18, 25),
        ];
        assert!(!can_book(&existing, Booking::new(1, 14, 19)));
        assert!(can_book(&existing, Booking::new(1, 15, 18)));
    }
}
