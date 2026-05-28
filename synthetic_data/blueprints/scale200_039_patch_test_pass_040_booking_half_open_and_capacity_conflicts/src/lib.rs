#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub attendees: u32,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32, attendees: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
            attendees,
        }
    }
}

pub fn can_book(existing: &[Booking], request: &Booking, room_capacity: u32) -> bool {
    if request.start > request.end || request.attendees > room_capacity {
        return false;
    }

    !existing.iter().any(|b| {
        b.room == request.room
            && request.start <= b.end
            && b.start <= request.end
            && b.attendees + request.attendees <= room_capacity
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = vec![Booking::new("A", 10, 20, 4)];
        let req = Booking::new("A", 20, 30, 3);
        assert!(can_book(&existing, &req, 10));
    }

    #[test]
    fn rejects_overlapping_when_combined_attendees_exceed_capacity() {
        let existing = vec![Booking::new("A", 10, 20, 7)];
        let req = Booking::new("A", 15, 18, 4);
        assert!(!can_book(&existing, &req, 10));
    }

    #[test]
    fn allows_overlapping_when_combined_attendees_fit_capacity() {
        let existing = vec![Booking::new("A", 10, 20, 3)];
        let req = Booking::new("A", 12, 18, 4);
        assert!(can_book(&existing, &req, 10));
    }

    #[test]
    fn ignores_other_rooms() {
        let existing = vec![Booking::new("B", 10, 20, 9)];
        let req = Booking::new("A", 12, 18, 9);
        assert!(can_book(&existing, &req, 10));
    }

    #[test]
    fn rejects_zero_length_booking() {
        let existing = vec![Booking::new("A", 10, 20, 1)];
        let req = Booking::new("A", 15, 15, 1);
        assert!(!can_book(&existing, &req, 10));
    }

    #[test]
    fn rejects_if_multiple_overlaps_push_total_over_capacity() {
        let existing = vec![
            Booking::new("A", 10, 16, 4),
            Booking::new("A", 14, 20, 5),
        ];
        let req = Booking::new("A", 15, 17, 2);
        assert!(!can_book(&existing, &req, 10));
    }
}
