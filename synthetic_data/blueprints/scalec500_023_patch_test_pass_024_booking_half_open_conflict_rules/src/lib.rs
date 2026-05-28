#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

pub fn has_conflict(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start >= end {
        return true;
    }

    existing.iter().any(|b| {
        b.room == room
            && !b.cancelled
            && start <= b.end
            && b.start <= end
    })
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    !has_conflict(existing, room, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &str, start: u32, end: u32, cancelled: bool) -> Booking {
        Booking {
            room: room.to_string(),
            start,
            end,
            cancelled,
        }
    }

    #[test]
    fn rejects_invalid_or_zero_length_requests() {
        let existing = vec![b("A", 10, 20, false)];
        assert!(has_conflict(&existing, "A", 15, 15));
        assert!(has_conflict(&existing, "A", 18, 12));
        assert!(!can_book(&existing, "A", 15, 15));
    }

    #[test]
    fn touching_endpoint_is_allowed_for_half_open_intervals() {
        let existing = vec![b("A", 10, 20, false)];
        assert!(!has_conflict(&existing, "A", 20, 25));
        assert!(!has_conflict(&existing, "A", 5, 10));
        assert!(can_book(&existing, "A", 20, 25));
    }

    #[test]
    fn overlapping_inside_or_covering_existing_conflicts() {
        let existing = vec![b("A", 10, 20, false)];
        assert!(has_conflict(&existing, "A", 12, 18));
        assert!(has_conflict(&existing, "A", 5, 12));
        assert!(has_conflict(&existing, "A", 18, 22));
        assert!(has_conflict(&existing, "A", 5, 25));
    }

    #[test]
    fn ignores_cancelled_bookings_and_other_rooms() {
        let existing = vec![
            b("A", 10, 20, true),
            b("B", 12, 18, false),
            b("A", 30, 40, false),
        ];
        assert!(!has_conflict(&existing, "A", 12, 18));
        assert!(!has_conflict(&existing, "A", 20, 30));
        assert!(has_conflict(&existing, "A", 35, 36));
    }

    #[test]
    fn any_active_overlap_in_same_room_blocks_booking() {
        let existing = vec![
            b("A", 0, 5, false),
            b("A", 5, 10, true),
            b("A", 10, 15, false),
        ];
        assert!(has_conflict(&existing, "A", 4, 11));
        assert!(!has_conflict(&existing, "A", 5, 10));
    }
}
