#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
    pub attendees: u32,
}

impl Booking {
    pub fn new(room: &'static str, start: u32, end: u32, attendees: u32) -> Self {
        Self { room, start, end, attendees }
    }
}

pub fn can_book(existing: &[Booking], candidate: &Booking, room_capacity: u32, maintenance: &[(u32, u32)]) -> bool {
    if candidate.start > candidate.end || candidate.attendees > room_capacity {
        return false;
    }

    for &(start, end) in maintenance {
        if candidate.start <= end && start <= candidate.end {
            return false;
        }
    }

    for booking in existing {
        if booking.room == candidate.room && candidate.start <= booking.end && booking.start <= candidate.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_to_back_bookings_are_allowed() {
        let existing = [Booking::new("A", 60, 120, 4)];
        let candidate = Booking::new("A", 120, 180, 3);
        assert!(can_book(&existing, &candidate, 10, &[]));
    }

    #[test]
    fn overlapping_same_room_is_rejected() {
        let existing = [Booking::new("A", 60, 120, 4)];
        let candidate = Booking::new("A", 119, 180, 3);
        assert!(!can_book(&existing, &candidate, 10, &[]));
    }

    #[test]
    fn different_rooms_do_not_conflict() {
        let existing = [Booking::new("A", 60, 120, 4)];
        let candidate = Booking::new("B", 90, 110, 3);
        assert!(can_book(&existing, &candidate, 10, &[]));
    }

    #[test]
    fn zero_length_booking_is_invalid() {
        let candidate = Booking::new("A", 100, 100, 1);
        assert!(!can_book(&[], &candidate, 10, &[]));
    }

    #[test]
    fn over_capacity_booking_is_invalid() {
        let candidate = Booking::new("A", 100, 130, 11);
        assert!(!can_book(&[], &candidate, 10, &[]));
    }

    #[test]
    fn touching_maintenance_window_is_allowed() {
        let candidate = Booking::new("A", 120, 180, 2);
        let maintenance = [(60, 120)];
        assert!(can_book(&[], &candidate, 10, &maintenance));
    }

    #[test]
    fn overlap_with_maintenance_is_rejected() {
        let candidate = Booking::new("A", 119, 180, 2);
        let maintenance = [(60, 120)];
        assert!(!can_book(&[], &candidate, 10, &maintenance));
    }

    #[test]
    fn invalid_reversed_range_is_rejected() {
        let candidate = Booking::new("A", 200, 150, 2);
        assert!(!can_book(&[], &candidate, 10, &[]));
    }
}
