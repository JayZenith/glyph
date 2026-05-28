#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
            canceled: false,
        }
    }
}

pub fn conflicts(existing: &[Booking], candidate: &Booking) -> bool {
    existing.iter().any(|b| {
        b.room == candidate.room
            && !b.canceled
            && candidate.start <= b.end
            && b.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_conflict_for_same_room() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 20, 25);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn actual_overlap_conflicts_for_same_room() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 19, 25);
        assert!(conflicts(&existing, &candidate));
    }

    #[test]
    fn contained_interval_conflicts() {
        let existing = vec![Booking::new("A", 10, 30)];
        let candidate = Booking::new("A", 12, 15);
        assert!(conflicts(&existing, &candidate));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("B", 15, 18);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn canceled_bookings_do_not_conflict() {
        let mut canceled = Booking::new("A", 10, 20);
        canceled.canceled = true;
        let existing = vec![canceled];
        let candidate = Booking::new("A", 15, 18);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn zero_length_candidate_never_conflicts() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 15, 15);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn invalid_existing_booking_is_ignored() {
        let existing = vec![Booking::new("A", 30, 10)];
        let candidate = Booking::new("A", 12, 15);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn any_valid_conflicting_booking_triggers_conflict() {
        let mut canceled = Booking::new("A", 10, 50);
        canceled.canceled = true;
        let invalid = Booking::new("A", 40, 35);
        let overlap = Booking::new("A", 20, 30);
        let existing = vec![canceled, invalid, overlap];
        let candidate = Booking::new("A", 25, 40);
        assert!(conflicts(&existing, &candidate));
    }
}
