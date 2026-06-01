#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn has_conflict(existing: &[Booking], requested: &Booking) -> bool {
    if requested.cancelled || requested.start > requested.end {
        return false;
    }

    existing.iter().any(|b| {
        if b.cancelled || b.room != requested.room {
            return false;
        }

        b.start <= requested.end && requested.start <= b.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_endpoints_do_not_conflict_in_same_room() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!has_conflict(&existing, &Booking::new("A", 20, 25)));
        assert!(!has_conflict(&existing, &Booking::new("A", 5, 10)));
    }

    #[test]
    fn proper_overlap_and_containment_conflict() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(has_conflict(&existing, &Booking::new("A", 19, 22)));
        assert!(has_conflict(&existing, &Booking::new("A", 12, 18)));
        assert!(has_conflict(&existing, &Booking::new("A", 5, 25)));
    }

    #[test]
    fn different_rooms_and_cancelled_entries_are_ignored() {
        let existing = vec![
            Booking::new("A", 10, 20),
            Booking::cancelled("A", 15, 30),
            Booking::new("B", 12, 18),
        ];
        assert!(!has_conflict(&existing, &Booking::new("B", 18, 22)));
        assert!(!has_conflict(&existing, &Booking::new("C", 12, 18)));
    }

    #[test]
    fn zero_length_request_is_invalid_and_never_conflicts() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!has_conflict(&existing, &Booking::new("A", 15, 15)));
    }

    #[test]
    fn invalid_reversed_request_is_also_non_conflicting() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!has_conflict(&existing, &Booking::new("A", 30, 10)));
    }
}
