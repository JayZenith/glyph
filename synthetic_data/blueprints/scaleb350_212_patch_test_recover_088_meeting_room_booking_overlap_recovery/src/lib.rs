#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start: u32,
    pub end: u32,
}

pub fn can_book(existing: &[Booking], room: &str, start: u32, end: u32) -> bool {
    for b in existing {
        if b.room != room {
            continue;
        }
        if start <= b.end && end >= b.start {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Booking> {
        vec![
            Booking { room: "A", start: 10, end: 20 },
            Booking { room: "A", start: 30, end: 40 },
            Booking { room: "B", start: 15, end: 25 },
        ]
    }

    #[test]
    fn rejects_overlap_in_same_room() {
        let existing = sample();
        assert!(!can_book(&existing, "A", 18, 22));
        assert!(!can_book(&existing, "A", 35, 36));
    }

    #[test]
    fn allows_different_room_and_back_to_back() {
        let existing = sample();
        assert!(can_book(&existing, "B", 26, 30));
        assert!(can_book(&existing, "A", 20, 30));
        assert!(can_book(&existing, "A", 40, 45));
    }

    #[test]
    fn rejects_invalid_or_containing_intervals() {
        let existing = sample();
        assert!(!can_book(&existing, "A", 22, 22));
        assert!(!can_book(&existing, "A", 25, 24));
        assert!(!can_book(&existing, "A", 5, 50));
    }
}
