#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
            cancelled: false,
        }
    }
}

pub fn has_conflict(existing: &[Booking], candidate: &Booking) -> bool {
    existing.iter().any(|b| {
        b.room == candidate.room
            && !b.cancelled
            && candidate.start <= b.end
            && b.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active(room: &str, start: u32, end: u32) -> Booking {
        Booking::new(room, start, end)
    }

    fn cancelled(room: &str, start: u32, end: u32) -> Booking {
        let mut b = Booking::new(room, start, end);
        b.cancelled = true;
        b
    }

    #[test]
    fn rejects_real_overlap_in_same_room() {
        let existing = vec![active("A", 10, 20), active("A", 30, 40)];
        let candidate = active("A", 18, 22);
        assert!(has_conflict(&existing, &candidate));
    }

    #[test]
    fn adjacent_intervals_do_not_conflict() {
        let existing = vec![active("A", 10, 20)];
        let candidate = active("A", 20, 25);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![active("A", 10, 20)];
        let candidate = active("B", 15, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn cancelled_bookings_are_ignored() {
        let existing = vec![cancelled("A", 10, 20)];
        let candidate = active("A", 15, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn invalid_candidate_with_zero_length_never_conflicts() {
        let existing = vec![active("A", 10, 20)];
        let candidate = active("A", 12, 12);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn invalid_existing_ranges_are_ignored() {
        let existing = vec![active("A", 50, 40), active("A", 60, 60)];
        let candidate = active("A", 45, 55);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn contained_interval_conflicts() {
        let existing = vec![active("A", 10, 30)];
        let candidate = active("A", 12, 18);
        assert!(has_conflict(&existing, &candidate));
    }
}
