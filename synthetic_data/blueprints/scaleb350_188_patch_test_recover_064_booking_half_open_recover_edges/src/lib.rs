#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

pub fn can_book(existing: &[Booking], requested: Booking) -> bool {
    if requested.start > requested.end {
        return false;
    }

    for slot in existing {
        if overlaps(*slot, requested) {
            return false;
        }
    }
    true
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_overlapping_middle() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(15, 18)));
    }

    #[test]
    fn allows_back_to_back_intervals() {
        let existing = [Booking::new(10, 20)];
        assert!(can_book(&existing, Booking::new(20, 25)));
        assert!(can_book(&existing, Booking::new(5, 10)));
    }

    #[test]
    fn rejects_container_and_contained() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(8, 22)));
        assert!(!can_book(&existing, Booking::new(12, 19)));
    }

    #[test]
    fn rejects_zero_length_request() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(30, 30)));
    }

    #[test]
    fn rejects_reversed_interval() {
        let existing = [Booking::new(10, 20)];
        assert!(!can_book(&existing, Booking::new(40, 35)));
    }

    #[test]
    fn scans_all_existing_bookings() {
        let existing = [Booking::new(0, 5), Booking::new(8, 12), Booking::new(20, 30)];
        assert!(!can_book(&existing, Booking::new(12, 21)));
        assert!(can_book(&existing, Booking::new(5, 8)));
    }
}
