#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
}

pub fn conflicts(existing: &[Booking], candidate: &Booking) -> Vec<Booking> {
    existing
        .iter()
        .filter(|b| {
            b.room == candidate.room
                && candidate.start <= b.end
                && b.start <= candidate.end
        })
        .cloned()
        .collect()
}

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }
    conflicts(existing, candidate).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &str, start: u32, end: u32) -> Booking {
        Booking {
            room: room.to_string(),
            start,
            end,
        }
    }

    #[test]
    fn adjacent_same_room_is_allowed() {
        let existing = vec![b("A", 10, 20)];
        assert!(can_book(&existing, &b("A", 20, 25)));
        assert!(conflicts(&existing, &b("A", 20, 25)).is_empty());
    }

    #[test]
    fn overlapping_same_room_is_rejected() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, &b("A", 19, 21)));
    }

    #[test]
    fn contained_interval_conflicts() {
        let existing = vec![b("A", 10, 30)];
        let found = conflicts(&existing, &b("A", 12, 18));
        assert_eq!(found, vec![b("A", 10, 30)]);
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![b("A", 10, 20), b("B", 12, 18)];
        assert!(can_book(&existing, &b("C", 12, 18)));
    }

    #[test]
    fn zero_length_and_reversed_intervals_are_invalid() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, &b("A", 15, 15)));
        assert!(!can_book(&existing, &b("A", 16, 15)));
    }

    #[test]
    fn returns_all_conflicts_in_existing_order() {
        let existing = vec![b("A", 0, 5), b("A", 8, 10), b("A", 3, 9), b("B", 1, 7)];
        let found = conflicts(&existing, &b("A", 4, 8));
        assert_eq!(found, vec![b("A", 0, 5), b("A", 3, 9)]);
    }
}
