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
    existing.iter().any(|b| b.start <= candidate.end && candidate.start <= b.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_in_same_room_conflicts() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 15, 25);
        assert!(has_conflict(&existing, &candidate));
    }

    #[test]
    fn touching_endpoint_is_not_a_conflict() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("A", 20, 30);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![Booking::new("A", 10, 20)];
        let candidate = Booking::new("B", 12, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn cancelled_bookings_are_ignored() {
        let existing = vec![Booking::cancelled("A", 10, 20)];
        let candidate = Booking::new("A", 12, 18);
        assert!(!has_conflict(&existing, &candidate));
    }

    #[test]
    fn any_active_overlap_in_same_room_still_conflicts() {
        let existing = vec![
            Booking::cancelled("A", 5, 50),
            Booking::new("B", 12, 18),
            Booking::new("A", 30, 40),
            Booking::new("A", 14, 16),
        ];
        let candidate = Booking::new("A", 15, 22);
        assert!(has_conflict(&existing, &candidate));
    }
}
