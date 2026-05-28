#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    if start > end {
        return false;
    }

    !existing.iter().any(|b| {
        !b.cancelled && (b.room == room || b.start <= end) && start <= b.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Booking> {
        vec![
            Booking { room: "A", start: 10, end: 20, cancelled: false },
            Booking { room: "A", start: 25, end: 30, cancelled: true },
            Booking { room: "B", start: 10, end: 15, cancelled: false },
            Booking { room: "A", start: 30, end: 40, cancelled: false },
        ]
    }

    #[test]
    fn rejects_invalid_zero_or_reversed_ranges() {
        let s = sample();
        assert!(!can_book(&s, "A", 8, 8));
        assert!(!can_book(&s, "A", 9, 8));
    }

    #[test]
    fn allows_touching_edges_in_same_room() {
        let s = sample();
        assert!(can_book(&s, "A", 20, 30));
        assert!(can_book(&s, "A", 40, 45));
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let s = sample();
        assert!(!can_book(&s, "A", 19, 21));
        assert!(!can_book(&s, "A", 35, 41));
    }

    #[test]
    fn ignores_cancelled_bookings() {
        let s = sample();
        assert!(can_book(&s, "A", 25, 30));
    }

    #[test]
    fn ignores_other_rooms_even_when_times_overlap() {
        let s = sample();
        assert!(can_book(&s, "C", 12, 14));
        assert!(can_book(&s, "B", 15, 18));
    }
}
