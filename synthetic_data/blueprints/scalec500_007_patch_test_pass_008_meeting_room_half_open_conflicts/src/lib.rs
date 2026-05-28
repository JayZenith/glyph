#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
        }
    }
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    for b in existing {
        if b.room != room {
            continue;
        }

        if start <= b.end && b.start <= end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &str, start: u32, end: u32) -> Booking {
        Booking::new(room, start, end)
    }

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let existing = vec![b("A", 10, 20)];
        assert!(can_book(&existing, "A", 20, 25));
        assert!(can_book(&existing, "A", 5, 10));
    }

    #[test]
    fn overlapping_ranges_conflict() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, "A", 19, 21));
        assert!(!can_book(&existing, "A", 10, 20));
        assert!(!can_book(&existing, "A", 12, 18));
    }

    #[test]
    fn different_rooms_are_independent() {
        let existing = vec![b("A", 10, 20), b("B", 10, 20)];
        assert!(can_book(&existing, "C", 10, 20));
        assert!(can_book(&existing, "B", 20, 22));
    }

    #[test]
    fn zero_length_request_is_invalid() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, "A", 15, 15));
        assert!(!can_book(&existing, "A", 20, 20));
    }

    #[test]
    fn zero_length_existing_entries_are_ignored() {
        let existing = vec![b("A", 10, 10), b("A", 30, 30)];
        assert!(can_book(&existing, "A", 10, 12));
        assert!(can_book(&existing, "A", 29, 30));
    }

    #[test]
    fn room_filter_applies_before_overlap_check() {
        let existing = vec![b("A", 8, 12), b("B", 9, 17), b("A", 20, 30)];
        assert!(can_book(&existing, "B", 17, 20));
        assert!(!can_book(&existing, "A", 11, 21));
    }
}
