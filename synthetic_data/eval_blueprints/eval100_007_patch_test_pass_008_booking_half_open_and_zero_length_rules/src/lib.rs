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

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    for booking in existing {
        if booking.room != candidate.room {
            continue;
        }

        if candidate.start <= booking.end && booking.start <= candidate.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_inverted_interval() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(!can_book(&existing, Booking::new(1, 8, 7)));
    }

    #[test]
    fn allows_adjacent_booking_in_same_room() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(can_book(&existing, Booking::new(1, 20, 25)));
    }

    #[test]
    fn rejects_strict_overlap_in_same_room() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(!can_book(&existing, Booking::new(1, 19, 25)));
    }

    #[test]
    fn allows_overlap_in_different_room() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(can_book(&existing, Booking::new(2, 15, 18)));
    }

    #[test]
    fn rejects_zero_length_candidate() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(!can_book(&existing, Booking::new(1, 22, 22)));
    }

    #[test]
    fn ignores_zero_length_existing_hold() {
        let existing = [Booking::new(1, 15, 15)];
        assert!(can_book(&existing, Booking::new(1, 15, 18)));
    }

    #[test]
    fn rejects_candidate_containing_existing() {
        let existing = [Booking::new(1, 12, 18)];
        assert!(!can_book(&existing, Booking::new(1, 10, 20)));
    }
}
