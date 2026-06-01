#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Option<Self> {
        if start >= end {
            None
        } else {
            Some(Self { start, end })
        }
    }
}

pub fn can_book(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().all(|b| candidate.end < b.start || candidate.start > b.end)
}

pub fn first_conflict(existing: &[Booking], candidate: Booking) -> Option<Booking> {
    existing
        .iter()
        .copied()
        .find(|b| !(candidate.end < b.start || candidate.start > b.end))
}

pub fn insert_booking(existing: &mut Vec<Booking>, candidate: Booking) -> bool {
    if !can_book(existing, candidate) {
        return false;
    }
    existing.push(candidate);
    existing.sort_by_key(|b| b.start);
    true
}

pub fn merge_bookings(bookings: &[Booking]) -> Vec<Booking> {
    let mut items = bookings.to_vec();
    items.sort_by_key(|b| b.start);
    let mut merged: Vec<Booking> = Vec::new();

    for booking in items {
        if let Some(last) = merged.last_mut() {
            if booking.start <= last.end {
                if booking.end > last.end {
                    last.end = booking.end;
                }
            } else {
                merged.push(booking);
            }
        } else {
            merged.push(booking);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(start: u32, end: u32) -> Booking {
        Booking::new(start, end).unwrap()
    }

    #[test]
    fn booking_validation_rejects_empty_or_reversed() {
        assert_eq!(Booking::new(5, 5), None);
        assert_eq!(Booking::new(8, 3), None);
        assert_eq!(Booking::new(2, 3), Some(Booking { start: 2, end: 3 }));
    }

    #[test]
    fn touching_edges_do_not_conflict_for_half_open_ranges() {
        let existing = vec![b(10, 20), b(30, 40)];
        assert!(can_book(&existing, b(20, 30)));
        assert!(can_book(&existing, b(0, 10)));
        assert_eq!(first_conflict(&existing, b(20, 30)), None);
    }

    #[test]
    fn overlapping_candidate_reports_first_conflict() {
        let existing = vec![b(10, 20), b(25, 35), b(40, 50)];
        assert!(!can_book(&existing, b(19, 26)));
        assert_eq!(first_conflict(&existing, b(19, 26)), Some(b(10, 20)));
    }

    #[test]
    fn insert_keeps_bookings_sorted_and_rejects_overlap() {
        let mut existing = vec![b(20, 25), b(5, 10)];
        assert!(insert_booking(&mut existing, b(10, 20)));
        assert_eq!(existing, vec![b(5, 10), b(10, 20), b(20, 25)]);
        assert!(!insert_booking(&mut existing, b(9, 12)));
    }

    #[test]
    fn merge_overlaps_but_not_touching_ranges() {
        let merged = merge_bookings(&[b(5, 10), b(8, 12), b(12, 15), b(30, 35), b(34, 40)]);
        assert_eq!(merged, vec![b(5, 12), b(12, 15), b(30, 40)]);
    }
}
