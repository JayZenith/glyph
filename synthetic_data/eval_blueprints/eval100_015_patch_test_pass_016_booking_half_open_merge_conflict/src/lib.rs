#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn can_book(existing: &[Booking], request: Booking) -> bool {
    if request.start > request.end {
        return false;
    }

    let mut slots = existing.to_vec();
    slots.sort_by_key(|b| b.start);

    let mut merged: Vec<Booking> = Vec::new();
    for slot in slots {
        if let Some(last) = merged.last_mut() {
            if slot.start <= last.end {
                if slot.end > last.end {
                    last.end = slot.end;
                }
            } else {
                merged.push(slot);
            }
        } else {
            merged.push(slot);
        }
    }

    !merged.into_iter().any(|slot| overlaps(slot, request))
}

#[cfg(test)]
mod tests {
    use super::{can_book, Booking};

    #[test]
    fn touching_edges_do_not_conflict() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(can_book(&existing, Booking { start: 20, end: 25 }));
        assert!(can_book(&existing, Booking { start: 5, end: 10 }));
    }

    #[test]
    fn overlapping_ranges_conflict() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!can_book(&existing, Booking { start: 19, end: 21 }));
        assert!(!can_book(&existing, Booking { start: 10, end: 11 }));
        assert!(!can_book(&existing, Booking { start: 0, end: 30 }));
    }

    #[test]
    fn zero_length_request_is_invalid() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!can_book(&existing, Booking { start: 15, end: 15 }));
        assert!(!can_book(&existing, Booking { start: 20, end: 20 }));
    }

    #[test]
    fn invalid_reversed_request_is_rejected() {
        let existing = [Booking { start: 10, end: 20 }];
        assert!(!can_book(&existing, Booking { start: 30, end: 29 }));
    }

    #[test]
    fn merged_existing_spans_block_inner_request() {
        let existing = [
            Booking { start: 30, end: 40 },
            Booking { start: 10, end: 20 },
            Booking { start: 18, end: 25 },
        ];
        assert!(!can_book(&existing, Booking { start: 12, end: 24 }));
        assert!(can_book(&existing, Booking { start: 25, end: 30 }));
    }

    #[test]
    fn disjoint_existing_spans_allow_gap_booking() {
        let existing = [
            Booking { start: 0, end: 5 },
            Booking { start: 10, end: 15 },
            Booking { start: 20, end: 25 },
        ];
        assert!(can_book(&existing, Booking { start: 5, end: 10 }));
        assert!(can_book(&existing, Booking { start: 15, end: 20 }));
        assert!(!can_book(&existing, Booking { start: 14, end: 21 }));
    }
}
