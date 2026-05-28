#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    if candidate.start >= candidate.end {
        return false;
    }

    !existing.iter().any(|b| b.room == candidate.room && overlaps(b, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn booking(room: &'static str, start: u32, end: u32) -> Booking {
        Booking {
            room,
            start,
            end,
            canceled: false,
        }
    }

    #[test]
    fn rejects_same_room_overlap() {
        let existing = vec![booking("A", 10, 20)];
        assert!(!can_book(&existing, &booking("A", 15, 18)));
    }

    #[test]
    fn allows_adjacent_half_open_slots() {
        let existing = vec![booking("A", 10, 20)];
        assert!(can_book(&existing, &booking("A", 20, 25)));
        assert!(can_book(&existing, &booking("A", 5, 10)));
    }

    #[test]
    fn ignores_other_rooms() {
        let existing = vec![booking("A", 10, 20)];
        assert!(can_book(&existing, &booking("B", 15, 18)));
    }

    #[test]
    fn ignores_canceled_bookings_for_conflicts() {
        let existing = vec![Booking {
            room: "A",
            start: 10,
            end: 20,
            canceled: true,
        }];
        assert!(can_book(&existing, &booking("A", 12, 18)));
    }

    #[test]
    fn rejects_invalid_interval() {
        let existing = vec![booking("A", 10, 20)];
        assert!(!can_book(&existing, &booking("A", 9, 9)));
        assert!(!can_book(&existing, &booking("A", 11, 10)));
    }
}
