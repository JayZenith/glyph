#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub attendees: u32,
    pub cancelled: bool,
}

fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32, attendees: u32, capacity: u32) -> bool {
    if start >= end || attendees > capacity {
        return false;
    }

    for b in existing {
        if b.cancelled {
            continue;
        }
        if overlaps(b.start, b.end, start, end) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Booking> {
        vec![
            Booking { room: "oak", start: 10, end: 20, attendees: 4, cancelled: false },
            Booking { room: "oak", start: 30, end: 40, attendees: 2, cancelled: true },
            Booking { room: "pine", start: 15, end: 25, attendees: 8, cancelled: false },
        ]
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let bookings = sample();
        assert!(!can_book(&bookings, "oak", 18, 22, 3, 10));
    }

    #[test]
    fn allows_adjacent_slot_in_same_room() {
        let bookings = sample();
        assert!(can_book(&bookings, "oak", 20, 25, 3, 10));
    }

    #[test]
    fn ignores_other_rooms_when_checking_conflicts() {
        let bookings = sample();
        assert!(can_book(&bookings, "oak", 22, 28, 3, 10));
    }

    #[test]
    fn ignores_cancelled_bookings() {
        let bookings = sample();
        assert!(can_book(&bookings, "oak", 35, 38, 2, 10));
    }

    #[test]
    fn enforces_capacity_and_valid_interval() {
        let bookings = sample();
        assert!(!can_book(&bookings, "oak", 12, 12, 1, 10));
        assert!(!can_book(&bookings, "oak", 12, 14, 11, 10));
    }
}
