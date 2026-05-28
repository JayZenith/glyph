#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    !(a.end < b.start || b.end < a.start)
}

pub fn can_book(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    for b in existing {
        if b.cancelled {
            continue;
        }
        if overlaps(b, &request) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{can_book, Booking};

    fn b(room: &'static str, start: u32, end: u32) -> Booking {
        Booking { room, start, end, cancelled: false }
    }

    fn c(room: &'static str, start: u32, end: u32) -> Booking {
        Booking { room, start, end, cancelled: true }
    }

    #[test]
    fn allows_back_to_back_in_same_room() {
        let existing = vec![b("A", 10, 20)];
        assert!(can_book(&existing, b("A", 20, 30)));
        assert!(can_book(&existing, b("A", 0, 10)));
    }

    #[test]
    fn rejects_true_overlap_in_same_room() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, b("A", 19, 25)));
        assert!(!can_book(&existing, b("A", 5, 11)));
        assert!(!can_book(&existing, b("A", 10, 20)));
    }

    #[test]
    fn ignores_other_rooms_and_cancelled_bookings() {
        let existing = vec![b("A", 10, 20), c("B", 12, 18)];
        assert!(can_book(&existing, b("B", 12, 18)));
        assert!(can_book(&existing, b("C", 15, 16)));
    }

    #[test]
    fn rejects_zero_length_and_reversed_requests() {
        let existing = vec![b("A", 10, 20)];
        assert!(!can_book(&existing, b("A", 7, 7)));
        assert!(!can_book(&existing, b("A", 9, 8)));
    }
}
