#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

pub fn can_book(existing: &[Booking], request: &Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    for slot in existing {
        if request.start <= slot.end && slot.start <= request.end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end)
    }

    #[test]
    fn allows_non_overlapping_gap() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(can_book(&existing, &b(21, 29)));
    }

    #[test]
    fn allows_back_to_back_at_endpoints() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(can_book(&existing, &b(20, 30)));
        assert!(can_book(&existing, &b(40, 45)));
        assert!(can_book(&existing, &b(5, 10)));
    }

    #[test]
    fn rejects_true_overlap_and_containment() {
        let existing = [b(10, 20), b(30, 40)];
        assert!(!can_book(&existing, &b(15, 18)));
        assert!(!can_book(&existing, &b(18, 31)));
        assert!(!can_book(&existing, &b(0, 50)));
    }

    #[test]
    fn rejects_zero_length_inside_existing_interval() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, &b(15, 15)));
    }

    #[test]
    fn allows_zero_length_on_free_boundary() {
        let existing = [b(10, 20), b(25, 30)];
        assert!(can_book(&existing, &b(20, 20)));
        assert!(can_book(&existing, &b(22, 22)));
    }

    #[test]
    fn rejects_invalid_reversed_interval() {
        let existing = [b(10, 20)];
        assert!(!can_book(&existing, &b(9, 8)));
    }
}
