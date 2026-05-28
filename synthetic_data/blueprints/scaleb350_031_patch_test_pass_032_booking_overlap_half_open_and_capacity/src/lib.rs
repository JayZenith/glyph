#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: &Booking, room_capacity: usize) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    let conflicts = existing
        .iter()
        .filter(|b| b.room == candidate.room)
        .filter(|b| overlaps(b, candidate))
        .count();

    conflicts < room_capacity
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &'static str, start: u32, end: u32) -> Booking {
        Booking {
            room,
            start,
            end,
            canceled: false,
        }
    }

    fn canceled(room: &'static str, start: u32, end: u32) -> Booking {
        Booking {
            room,
            start,
            end,
            canceled: true,
        }
    }

    #[test]
    fn adjacent_bookings_do_not_conflict_in_same_room() {
        let existing = vec![b("A", 10, 20)];
        assert!(can_book(&existing, &b("A", 20, 30), 1));
    }

    #[test]
    fn canceled_bookings_do_not_consume_capacity() {
        let existing = vec![canceled("A", 10, 20)];
        assert!(can_book(&existing, &b("A", 12, 18), 1));
    }

    #[test]
    fn different_rooms_are_independent() {
        let existing = vec![b("B", 10, 20), b("A", 0, 100)];
        assert!(!can_book(&existing, &b("A", 30, 40), 1));
        assert!(can_book(&existing, &b("C", 30, 40), 1));
    }

    #[test]
    fn capacity_allows_parallel_overlaps_up_to_limit() {
        let existing = vec![b("A", 10, 20), b("A", 12, 18)];
        assert!(can_book(&existing, &b("A", 13, 17), 3));
        assert!(!can_book(&existing, &b("A", 13, 17), 2));
    }

    #[test]
    fn invalid_or_empty_candidate_intervals_are_rejected() {
        assert!(!can_book(&[], &b("A", 9, 9), 1));
        assert!(!can_book(&[], &b("A", 10, 9), 1));
    }
}
