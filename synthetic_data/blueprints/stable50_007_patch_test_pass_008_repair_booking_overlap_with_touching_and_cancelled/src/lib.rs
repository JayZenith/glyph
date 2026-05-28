#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

impl Booking {
    pub fn new(room: &'static str, start: u32, end: u32) -> Self {
        Self {
            room,
            start,
            end,
            cancelled: false,
        }
    }

    pub fn cancelled(room: &'static str, start: u32, end: u32) -> Self {
        Self {
            room,
            start,
            end,
            cancelled: true,
        }
    }
}

pub fn has_conflict(existing: &[Booking], candidate: &Booking) -> bool {
    existing.iter().any(|b| {
        if b.cancelled || candidate.cancelled {
            return false;
        }
        if b.room != candidate.room {
            return false;
        }

        candidate.start <= b.end && b.start <= candidate.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_do_not_conflict_in_same_room() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 20, 30);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn overlap_inside_same_room_conflicts() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 19, 25);
        assert!(has_conflict(&existing, &candidate));
    }

    #[test]
    fn containment_conflicts() {
        let existing = vec![Booking::new("A", 10, 30)];
        let candidate = Booking::new("A", 12, 18);
        assert!(has_conflict(&existing, &candidate));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("B", 15, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn cancelled_existing_booking_does_not_block() {
        let existing = vec![Booking::cancelled("A", 10, 20)];
        let candidate = Booking::new("A", 12, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn cancelled_candidate_never_conflicts() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::cancelled("A", 12, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn scans_all_existing_bookings() {
        let existing = vec![
            Booking::new("A", 0, 5),
            Booking::new("B", 8, 12),
            Booking::new("A", 20, 30),
        ];
        let candidate = Booking::new("A", 25, 28);
        assert!(has_conflict(&existing, &candidate));
    }
}
