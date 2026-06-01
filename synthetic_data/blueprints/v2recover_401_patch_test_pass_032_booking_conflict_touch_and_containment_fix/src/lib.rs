#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: u32,
}

impl Booking {
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start < end, "start must be before end");
        Self { start, end }
    }
}

pub fn overlaps(a: Booking, b: Booking) -> bool {
    a.start <= b.end && b.start <= a.end
}

pub fn has_conflict(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().copied().any(|slot| overlaps(slot, candidate))
}

pub fn insert_booking(existing: &mut Vec<Booking>, candidate: Booking) -> Result<(), &'static str> {
    if has_conflict(existing, candidate) {
        return Err("conflict");
    }
    existing.push(candidate);
    existing.sort_by_key(|b| b.start);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_edges_are_allowed() {
        let mut bookings = vec![Booking::new(10, 20)];
        assert_eq!(insert_booking(&mut bookings, Booking::new(20, 30)), Ok(()));
        assert_eq!(bookings, vec![Booking::new(10, 20), Booking::new(20, 30)]);
    }

    #[test]
    fn interior_overlap_is_rejected() {
        let mut bookings = vec![Booking::new(10, 20)];
        assert_eq!(insert_booking(&mut bookings, Booking::new(15, 18)), Err("conflict"));
        assert_eq!(bookings, vec![Booking::new(10, 20)]);
    }

    #[test]
    fn containment_and_partial_overlap_both_conflict() {
        let existing = vec![Booking::new(10, 20), Booking::new(30, 40)];
        assert!(has_conflict(&existing, Booking::new(5, 35)));
        assert!(has_conflict(&existing, Booking::new(18, 22)));
        assert!(!has_conflict(&existing, Booking::new(20, 30)));
    }

    #[test]
    fn insertion_keeps_sorted_order() {
        let mut bookings = vec![Booking::new(30, 40), Booking::new(50, 60)];
        assert_eq!(insert_booking(&mut bookings, Booking::new(10, 20)), Ok(()));
        assert_eq!(bookings, vec![Booking::new(10, 20), Booking::new(30, 40), Booking::new(50, 60)]);
    }

    #[test]
    fn chain_of_non_overlapping_adjacent_slots_is_valid() {
        let mut bookings = vec![Booking::new(0, 5), Booking::new(10, 15)];
        assert_eq!(insert_booking(&mut bookings, Booking::new(5, 10)), Ok(()));
        assert_eq!(bookings, vec![Booking::new(0, 5), Booking::new(5, 10), Booking::new(10, 15)]);
    }
}
