#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Booking {
    pub room: &'static str,
    pub start_min: u32,
    pub end_min: u32,
}

fn overlaps(a: &Booking, b: &Booking) -> bool {
    a.start_min <= b.end_min && b.start_min <= a.end_min
}

pub fn can_book(existing: &[Booking], candidate: &Booking) -> bool {
    existing
        .iter()
        .filter(|b| b.room == candidate.room)
        .all(|b| !overlaps(b, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_back_to_back_same_room() {
        let existing = [Booking {
            room: "A",
            start_min: 60,
            end_min: 120,
        }];
        let candidate = Booking {
            room: "A",
            start_min: 120,
            end_min: 180,
        };
        assert!(can_book(&existing, &candidate));
    }

    #[test]
    fn rejects_true_overlap_same_room() {
        let existing = [Booking {
            room: "A",
            start_min: 60,
            end_min: 120,
        }];
        let candidate = Booking {
            room: "A",
            start_min: 119,
            end_min: 150,
        };
        assert!(!can_book(&existing, &candidate));
    }

    #[test]
    fn allows_same_time_different_room() {
        let existing = [Booking {
            room: "A",
            start_min: 60,
            end_min: 120,
        }];
        let candidate = Booking {
            room: "B",
            start_min: 80,
            end_min: 100,
        };
        assert!(can_book(&existing, &candidate));
    }

    #[test]
    fn rejects_invalid_candidate_window() {
        let existing = [Booking {
            room: "A",
            start_min: 60,
            end_min: 120,
        }];
        let candidate = Booking {
            room: "A",
            start_min: 200,
            end_min: 200,
        };
        assert!(!can_book(&existing, &candidate));
    }

    #[test]
    fn rejects_when_candidate_contains_existing() {
        let existing = [Booking {
            room: "A",
            start_min: 100,
            end_min: 140,
        }];
        let candidate = Booking {
            room: "A",
            start_min: 90,
            end_min: 150,
        };
        assert!(!can_book(&existing, &candidate));
    }
}
