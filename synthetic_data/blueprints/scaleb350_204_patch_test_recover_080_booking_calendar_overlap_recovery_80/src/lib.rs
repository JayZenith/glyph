#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

pub fn is_available(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    for b in existing {
        if request.start >= b.start && request.start <= b.end {
            return false;
        }
        if request.end >= b.start && request.end <= b.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{is_available, Booking};

    #[test]
    fn allows_gap_and_touching_endpoints() {
        let existing = [Booking { start: 10, end: 20 }, Booking { start: 30, end: 40 }];
        assert!(is_available(&existing, Booking { start: 20, end: 30 }));
        assert!(is_available(&existing, Booking { start: 0, end: 10 }));
        assert!(is_available(&existing, Booking { start: 40, end: 50 }));
    }

    #[test]
    fn rejects_any_real_overlap() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!is_available(&existing, Booking { start: 15, end: 25 }));
        assert!(!is_available(&existing, Booking { start: 5, end: 12 }));
        assert!(!is_available(&existing, Booking { start: 12, end: 18 }));
    }

    #[test]
    fn rejects_containing_existing_booking() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!is_available(&existing, Booking { start: 5, end: 25 }));
    }

    #[test]
    fn rejects_invalid_or_zero_length_requests() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!is_available(&existing, Booking { start: 9, end: 9 }));
        assert!(!is_available(&existing, Booking { start: 21, end: 20 }));
    }
}
