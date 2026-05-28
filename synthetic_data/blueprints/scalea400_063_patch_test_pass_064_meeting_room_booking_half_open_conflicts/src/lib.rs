#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: String,
    pub start: u32,
    pub end: u32,
    pub canceled: bool,
}

impl Booking {
    pub fn new(room: &str, start: u32, end: u32) -> Self {
        Self {
            room: room.to_string(),
            start,
            end,
            canceled: false,
        }
    }

    pub fn canceled(mut self) -> Self {
        self.canceled = true;
        self
    }
}

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    for b in existing {
        if b.canceled {
            continue;
        }
        if b.room != candidate.room {
            continue;
        }
        if candidate.start >= b.start && candidate.start <= b.end {
            return false;
        }
        if candidate.end >= b.start && candidate.end <= b.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_bookings() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(can_book(&existing, &Booking::new("A", 20, 30)));
    }

    #[test]
    fn rejects_containment_overlap() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!can_book(&existing, &Booking::new("A", 5, 25)));
    }

    #[test]
    fn ignores_other_rooms_and_canceled_bookings() {
        let existing = vec![
            Booking::new("B", 10, 20),
            Booking::new("A", 12, 18).canceled(),
        ];
        assert!(can_book(&existing, &Booking::new("A", 12, 18)));
    }

    #[test]
    fn zero_length_booking_is_invalid() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!can_book(&existing, &Booking::new("A", 15, 15)));
    }

    #[test]
    fn overlapping_middle_section_is_rejected() {
        let existing = vec![Booking::new("A", 10, 20)];
        assert!(!can_book(&existing, &Booking::new("A", 18, 22)));
    }
}
