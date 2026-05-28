#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Default)]
pub struct Calendar {
    bookings: Vec<Booking>,
}

impl Calendar {
    pub fn new() -> Self {
        Self { bookings: Vec::new() }
    }

    pub fn bookings(&self) -> &[Booking] {
        &self.bookings
    }

    pub fn book(&mut self, start: u32, end: u32) -> bool {
        if start > end {
            return false;
        }

        for existing in &self.bookings {
            if start >= existing.start && start <= existing.end {
                return false;
            }
            if end >= existing.start && end <= existing.end {
                return false;
            }
        }

        self.bookings.push(Booking { start, end });
        self.bookings.sort_by_key(|b| (b.start, b.end));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_adjacent_bookings() {
        let mut cal = Calendar::new();
        assert!(cal.book(10, 20));
        assert!(cal.book(20, 30));
        assert!(cal.book(5, 10));
    }

    #[test]
    fn rejects_partial_overlap_from_left_or_right() {
        let mut cal = Calendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(!cal.book(5, 15));
    }

    #[test]
    fn rejects_containing_and_contained_intervals() {
        let mut cal = Calendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(12, 18));
        assert!(!cal.book(0, 30));
    }

    #[test]
    fn rejects_zero_length_and_reversed_intervals() {
        let mut cal = Calendar::new();
        assert!(!cal.book(7, 7));
        assert!(!cal.book(9, 3));
        assert!(cal.book(1, 2));
    }

    #[test]
    fn keeps_bookings_sorted_after_successful_inserts() {
        let mut cal = Calendar::new();
        assert!(cal.book(30, 40));
        assert!(cal.book(10, 20));
        assert!(cal.book(20, 25));
        assert_eq!(
            cal.bookings(),
            &[
                Booking { start: 10, end: 20 },
                Booking { start: 20, end: 25 },
                Booking { start: 30, end: 40 },
            ]
        );
    }
}
