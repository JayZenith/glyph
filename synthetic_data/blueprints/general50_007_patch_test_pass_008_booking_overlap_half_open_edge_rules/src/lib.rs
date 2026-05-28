#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

pub fn conflicts(existing: &[Booking], candidate: &Booking) -> bool {
    existing.iter().any(|b| {
        b.room == candidate.room && ranges_overlap(b.start, b.end, candidate.start, candidate.end)
    })
}

fn ranges_overlap(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(room: &'static str, start: u32, end: u32, cancelled: bool) -> Booking {
        Booking { room, start, end, cancelled }
    }

    #[test]
    fn same_room_overlapping_intervals_conflict() {
        let existing = [b("A", 10, 20, false)];
        let candidate = b("A", 15, 25, false);
        assert!(conflicts(&existing, &candidate));
    }

    #[test]
    fn touching_endpoint_does_not_conflict_for_half_open_intervals() {
        let existing = [b("A", 10, 20, false)];
        let candidate = b("A", 20, 30, false);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = [b("A", 10, 20, false)];
        let candidate = b("B", 15, 18, false);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn cancelled_existing_booking_is_ignored() {
        let existing = [b("A", 10, 20, true)];
        let candidate = b("A", 12, 18, false);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn cancelled_candidate_never_conflicts() {
        let existing = [b("A", 10, 20, false)];
        let candidate = b("A", 12, 18, true);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn invalid_candidate_interval_never_conflicts() {
        let existing = [b("A", 10, 20, false)];
        let candidate = b("A", 18, 18, false);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn invalid_existing_interval_is_ignored() {
        let existing = [b("A", 30, 30, false), b("A", 40, 35, false)];
        let candidate = b("A", 30, 31, false);
        assert!(!conflicts(&existing, &candidate));
    }

    #[test]
    fn overlap_among_multiple_bookings_still_detected() {
        let existing = [
            b("A", 0, 5, false),
            b("A", 8, 10, true),
            b("B", 12, 20, false),
            b("A", 21, 30, false),
        ];
        let candidate = b("A", 25, 27, false);
        assert!(conflicts(&existing, &candidate));
    }
}
