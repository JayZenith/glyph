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

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    for booking in existing {
        if booking.room != candidate.room {
            continue;
        }
        if candidate.start <= booking.end && booking.start <= candidate.end {
            return false;
        }
    }
    true
}

pub fn add_booking(existing: &mut Vec<Booking>, candidate: Booking) -> bool {
    if can_book(existing, &candidate) {
        existing.push(candidate);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed() -> Vec<Booking> {
        vec![
            Booking::new("red", 10, 20),
            Booking::new("red", 30, 40),
            Booking::new("blue", 10, 20),
        ]
    }

    #[test]
    fn overlap_in_same_room_is_rejected() {
        let bookings = seed();
        assert!(!can_book(&bookings, &Booking::new("red", 15, 18)));
        assert!(!can_book(&bookings, &Booking::new("red", 18, 35)));
    }

    #[test]
    fn different_room_does_not_conflict() {
        let bookings = seed();
        assert!(can_book(&bookings, &Booking::new("green", 15, 18)));
    }

    #[test]
    fn touching_endpoints_do_not_conflict_for_half_open_ranges() {
        let bookings = seed();
        assert!(can_book(&bookings, &Booking::new("red", 20, 30)));
        assert!(can_book(&bookings, &Booking::new("red", 40, 45)));
        assert!(!can_book(&bookings, &Booking::new("red", 19, 30)));
    }

    #[test]
    fn invalid_or_empty_ranges_are_rejected() {
        let mut bookings = seed();
        assert!(!can_book(&bookings, &Booking::new("red", 25, 25)));
        assert!(!can_book(&bookings, &Booking::new("red", 50, 45)));
        assert!(!add_booking(&mut bookings, Booking::new("green", 7, 7)));
        assert_eq!(bookings.len(), 3);
    }

    #[test]
    fn valid_booking_is_added() {
        let mut bookings = seed();
        assert!(add_booking(&mut bookings, Booking::new("red", 20, 30)));
        assert_eq!(bookings.len(), 4);
        assert_eq!(bookings[3], Booking::new("red", 20, 30));
    }
}
