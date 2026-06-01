#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
}

pub fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], request: &Booking) -> bool {
    if request.start > request.end {
        return true;
    }

    for b in existing {
        if overlaps(b, request) {
            return false;
        }
    }
    true
}

pub fn conflicting_bookings(existing: &[Booking], request: &Booking) -> Vec<Booking> {
    let mut out = Vec::new();
    for b in existing {
        if overlaps(b, request) {
            out.push(request.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bk(room: &str, start: u32, end: u32) -> Booking {
        Booking {
            room: room.to_string(),
            start,
            end,
        }
    }

    #[test]
    fn edge_touching_is_allowed_for_same_room() {
        let existing = vec![bk("A", 10, 20)];
        assert!(can_book(&existing, &bk("A", 20, 25)));
        assert!(can_book(&existing, &bk("A", 5, 10)));
    }

    #[test]
    fn zero_length_request_is_invalid() {
        let existing = vec![bk("A", 10, 20)];
        assert!(!can_book(&existing, &bk("A", 15, 15)));
        assert!(conflicting_bookings(&existing, &bk("A", 15, 15)).is_empty());
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = vec![bk("A", 10, 20), bk("B", 12, 18)];
        assert!(can_book(&existing, &bk("C", 12, 18)));
        assert!(conflicting_bookings(&existing, &bk("C", 12, 18)).is_empty());
    }

    #[test]
    fn same_room_overlap_and_containment_are_blocked() {
        let existing = vec![bk("A", 10, 20), bk("A", 30, 40)];
        assert!(!can_book(&existing, &bk("A", 12, 18)));
        assert!(!can_book(&existing, &bk("A", 8, 22)));
        assert!(!can_book(&existing, &bk("A", 18, 35)));
    }

    #[test]
    fn conflicts_return_existing_matches_in_order() {
        let existing = vec![bk("A", 10, 20), bk("B", 15, 25), bk("A", 18, 22), bk("A", 30, 40)];
        let got = conflicting_bookings(&existing, &bk("A", 19, 35));
        assert_eq!(got, vec![bk("A", 10, 20), bk("A", 18, 22), bk("A", 30, 40)]);
    }

    #[test]
    fn invalid_reversed_request_is_rejected_without_conflicts() {
        let existing = vec![bk("A", 10, 20)];
        assert!(!can_book(&existing, &bk("A", 25, 20)));
        assert!(conflicting_bookings(&existing, &bk("A", 25, 20)).is_empty());
    }
}
