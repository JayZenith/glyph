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

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    if candidate.start > candidate.end {
        return false;
    }

    existing.iter().all(|b| !overlaps(b, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_adjacent_in_same_room() {
        let existing = vec![Booking::new("alpha", 10, 20)];
        assert!(can_book(&existing, &Booking::new("alpha", 20, 25)));
        assert!(can_book(&existing, &Booking::new("alpha", 5, 10)));
    }

    #[test]
    fn rejects_real_overlap_in_same_room() {
        let existing = vec![Booking::new("alpha", 10, 20)];
        assert!(!can_book(&existing, &Booking::new("alpha", 19, 22)));
        assert!(!can_book(&existing, &Booking::new("alpha", 12, 18)));
    }

    #[test]
    fn ignores_other_rooms() {
        let existing = vec![
            Booking::new("alpha", 10, 20),
            Booking::new("beta", 15, 30),
        ];
        assert!(can_book(&existing, &Booking::new("gamma", 15, 18)));
        assert!(can_book(&existing, &Booking::new("beta", 30, 40)));
    }

    #[test]
    fn rejects_zero_length_and_reversed_ranges() {
        let existing = vec![Booking::new("alpha", 10, 20)];
        assert!(!can_book(&existing, &Booking::new("alpha", 7, 7)));
        assert!(!can_book(&existing, &Booking::new("alpha", 9, 8)));
    }
}
