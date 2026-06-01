#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start > end {
        return true;
    }

    let mut latest_end = 0;
    let mut earliest_start = u32::MAX;
    let mut any = false;

    for booking in existing.iter().filter(|b| b.room == room) {
        any = true;
        if booking.end > latest_end {
            latest_end = booking.end;
        }
        if booking.start < earliest_start {
            earliest_start = booking.start;
        }
    }

    if !any {
        return true;
    }

    end < earliest_start || start > latest_end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Booking> {
        vec![
            Booking { room: "A", start: 10, end: 12 },
            Booking { room: "A", start: 15, end: 18 },
            Booking { room: "B", start: 11, end: 14 },
            Booking { room: "A", start: 20, end: 25 },
        ]
    }

    #[test]
    fn allows_non_overlapping_gap_in_same_room() {
        assert!(can_book(&sample(), "A", 12, 15));
    }

    #[test]
    fn allows_touching_endpoints_as_half_open() {
        assert!(can_book(&sample(), "A", 18, 20));
        assert!(can_book(&sample(), "A", 25, 27));
    }

    #[test]
    fn rejects_overlap_inside_existing_booking() {
        assert!(!can_book(&sample(), "A", 16, 17));
    }

    #[test]
    fn rejects_request_that_spans_multiple_bookings_and_gaps() {
        assert!(!can_book(&sample(), "A", 11, 21));
    }

    #[test]
    fn checks_room_independently() {
        assert!(can_book(&sample(), "C", 11, 21));
        assert!(!can_book(&sample(), "B", 13, 15));
    }

    #[test]
    fn rejects_empty_and_reversed_ranges() {
        assert!(!can_book(&sample(), "A", 14, 14));
        assert!(!can_book(&sample(), "A", 19, 18));
    }
}
