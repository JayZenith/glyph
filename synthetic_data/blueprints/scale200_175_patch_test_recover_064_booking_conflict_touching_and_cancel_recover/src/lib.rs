#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start >= end {
        return false;
    }

    for booking in existing {
        if booking.room == room && overlaps(start, end, booking.start, booking.end) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &str, start: u32, end: u32) -> Booking {
        Booking {
            room: room.to_string(),
            start,
            end,
            cancelled: false,
        }
    }

    fn cancelled(room: &str, start: u32, end: u32) -> Booking {
        Booking {
            room: room.to_string(),
            start,
            end,
            cancelled: true,
        }
    }

    #[test]
    fn rejects_invalid_interval() {
        assert!(!can_book(&[], "A", 10, 10));
        assert!(!can_book(&[], "A", 12, 11));
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let items = vec![b("A", 10, 20)];
        assert!(!can_book(&items, "A", 15, 18));
        assert!(!can_book(&items, "A", 5, 12));
        assert!(!can_book(&items, "A", 8, 25));
    }

    #[test]
    fn allows_touching_edges_in_same_room() {
        let items = vec![b("A", 10, 20)];
        assert!(can_book(&items, "A", 20, 25));
        assert!(can_book(&items, "A", 5, 10));
    }

    #[test]
    fn ignores_other_rooms() {
        let items = vec![b("A", 10, 20)];
        assert!(can_book(&items, "B", 15, 18));
    }

    #[test]
    fn ignores_cancelled_bookings_for_conflicts() {
        let items = vec![cancelled("A", 10, 20), b("B", 10, 20)];
        assert!(can_book(&items, "A", 12, 18));
        assert!(can_book(&items, "A", 20, 22));
    }

    #[test]
    fn mixed_active_and_cancelled_still_blocks_on_active_only() {
        let items = vec![cancelled("A", 10, 20), b("A", 30, 40)];
        assert!(can_book(&items, "A", 12, 18));
        assert!(!can_book(&items, "A", 35, 36));
        assert!(can_book(&items, "A", 40, 45));
    }
}
