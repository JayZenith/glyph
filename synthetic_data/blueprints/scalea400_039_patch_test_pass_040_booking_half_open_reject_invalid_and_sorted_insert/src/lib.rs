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
        let candidate = Booking { start, end };

        if self
            .bookings
            .iter()
            .any(|b| candidate.start <= b.end && b.start <= candidate.end)
        {
            return false;
        }

        self.bookings.push(candidate);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Booking, Calendar};

    #[test]
    fn touching_half_open_intervals_do_not_conflict() {
        let mut cal = Calendar::new();
        assert!(cal.book(10, 20));
        assert!(cal.book(20, 30));
        assert!(cal.book(5, 10));
    }

    #[test]
    fn overlapping_intervals_conflict() {
        let mut cal = Calendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(!cal.book(5, 11));
        assert!(!cal.book(10, 20));
    }

    #[test]
    fn invalid_or_empty_intervals_are_rejected() {
        let mut cal = Calendar::new();
        assert!(!cal.book(8, 8));
        assert!(!cal.book(9, 4));
        assert!(cal.book(1, 3));
        assert_eq!(cal.bookings(), &[Booking { start: 1, end: 3 }]);
    }

    #[test]
    fn accepted_bookings_remain_sorted_by_start() {
        let mut cal = Calendar::new();
        assert!(cal.book(20, 25));
        assert!(cal.book(5, 10));
        assert!(cal.book(12, 18));
        assert_eq!(
            cal.bookings(),
            &[
                Booking { start: 5, end: 10 },
                Booking { start: 12, end: 18 },
                Booking { start: 20, end: 25 },
            ]
        );
    }
}
