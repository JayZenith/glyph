#[derive(Debug, Default)]
pub struct BookingCalendar {
    slots: Vec<(u32, u32)>,
}

impl BookingCalendar {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    pub fn book(&mut self, start: u32, end: u32) -> bool {
        if start > end {
            return false;
        }

        for &(s, e) in &self.slots {
            if start >= s && start < e {
                return false;
            }
            if end > s && end <= e {
                return false;
            }
        }

        self.slots.push((start, end));
        self.slots.sort_unstable();
        true
    }

    pub fn bookings(&self) -> &[(u32, u32)] {
        &self.slots
    }
}

#[cfg(test)]
mod tests {
    use super::BookingCalendar;

    #[test]
    fn allows_adjacent_bookings() {
        let mut cal = BookingCalendar::new();
        assert!(cal.book(10, 20));
        assert!(cal.book(20, 30));
        assert!(cal.book(0, 10));
    }

    #[test]
    fn rejects_partial_overlap() {
        let mut cal = BookingCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(!cal.book(5, 12));
    }

    #[test]
    fn rejects_containing_interval() {
        let mut cal = BookingCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(5, 25));
    }

    #[test]
    fn rejects_zero_length_and_reversed() {
        let mut cal = BookingCalendar::new();
        assert!(!cal.book(8, 8));
        assert!(!cal.book(9, 4));
        assert!(cal.book(10, 12));
    }

    #[test]
    fn keeps_successful_bookings_sorted() {
        let mut cal = BookingCalendar::new();
        assert!(cal.book(30, 40));
        assert!(cal.book(10, 20));
        assert!(cal.book(20, 30));
        assert_eq!(cal.bookings(), &[(10, 20), (20, 30), (30, 40)]);
    }
}
