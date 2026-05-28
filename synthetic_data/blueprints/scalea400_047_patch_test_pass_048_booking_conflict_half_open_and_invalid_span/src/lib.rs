#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub room: u32,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

impl Booking {
    pub fn new(room: u32, start: u32, end: u32) -> Self {
        Self {
            room,
            start,
            end,
            canceled: false,
        }
    }
}

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| {
        !b.canceled
            && b.room == candidate.room
            && candidate.start <= b.end
            && b.start <= candidate.end
    })
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    !candidate.canceled && !has_conflict(existing, candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_bookings_do_not_conflict() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(can_book(&existing, Booking::new(1, 20, 25)));
        assert!(can_book(&existing, Booking::new(1, 5, 10)));
    }

    #[test]
    fn overlapping_bookings_conflict_in_same_room() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(has_conflict(&existing, Booking::new(1, 19, 21)));
        assert!(has_conflict(&existing, Booking::new(1, 10, 20)));
        assert!(has_conflict(&existing, Booking::new(1, 12, 18)));
    }

    #[test]
    fn different_rooms_and_canceled_entries_do_not_block() {
        let mut canceled = Booking::new(1, 10, 20);
        canceled.canceled = true;
        let existing = [canceled, Booking::new(2, 10, 20)];
        assert!(can_book(&existing, Booking::new(1, 12, 18)));
    }

    #[test]
    fn zero_or_reversed_length_candidates_are_rejected() {
        let existing = [Booking::new(1, 10, 20)];
        assert!(!can_book(&existing, Booking::new(1, 20, 20)));
        assert!(!can_book(&existing, Booking::new(1, 30, 25)));
        assert!(!can_book(&[], Booking::new(1, 5, 5)));
    }
}
