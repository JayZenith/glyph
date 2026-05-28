#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Result<Self, &'static str> {
        if start >= end {
            return Err("start must be before end");
        }
        Ok(Self { start, end })
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|slot| slot.end < candidate.start || candidate.end < slot.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn rejects_actual_overlap() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(!can_book(&existing, b(18, 22)));
        assert!(!can_book(&existing, b(35, 45)));
    }

    #[test]
    fn allows_touching_boundaries() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(can_book(&existing, b(20, 30)));
        assert!(can_book(&existing, b(0, 10)));
        assert!(can_book(&existing, b(40, 50)));
    }

    #[test]
    fn validates_booking_range() {
        assert!(Booking::new(5, 5).is_err());
        assert!(Booking::new(8, 3).is_err());
    }
}
