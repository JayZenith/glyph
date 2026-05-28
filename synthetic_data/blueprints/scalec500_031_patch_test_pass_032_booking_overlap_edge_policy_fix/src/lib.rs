#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub cancelled: bool,
}

impl Booking {
    pub fn new(room: &'static str, start: u32, end: u32) -> Self {
        Self { room, start, end, cancelled: false }
    }

    pub fn cancelled(room: &'static str, start: u32, end: u32) -> Self {
        Self { room, start, end, cancelled: true }
    }
}

pub fn can_book(existing: &[Booking], request: &Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    !existing.iter().any(|b| {
        b.room == request.room
            && overlaps(b.start, b.end, request.start, request.end)
    })
}

fn overlaps(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_end && b_start <= a_end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_same_room_is_allowed() {
        let existing = [Booking::new("A", 10, 20)];
        let req = Booking::new("A", 20, 30);
        assert!(can_book(&existing, &req));
    }

    #[test]
    fn overlapping_same_room_is_rejected() {
        let existing = [Booking::new("A", 10, 20)];
        let req = Booking::new("A", 19, 25);
        assert!(!can_book(&existing, &req));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = [Booking::new("A", 10, 20)];
        let req = Booking::new("B", 15, 18);
        assert!(can_book(&existing, &req));
    }

    #[test]
    fn cancelled_bookings_do_not_block() {
        let existing = [Booking::cancelled("A", 10, 20)];
        let req = Booking::new("A", 12, 14);
        assert!(can_book(&existing, &req));
    }

    #[test]
    fn zero_length_request_is_invalid() {
        let existing = [Booking::new("A", 1, 5)];
        let req = Booking::new("A", 7, 7);
        assert!(!can_book(&existing, &req));
    }

    #[test]
    fn reversed_request_is_invalid() {
        let existing = [Booking::new("A", 1, 5)];
        let req = Booking::new("A", 9, 3);
        assert!(!can_book(&existing, &req));
    }
}
