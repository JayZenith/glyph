pub fn has_conflict(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    existing.iter().any(|&(a, b)| start <= b && end >= a)
}

#[cfg(test)]
mod tests {
    use super::has_conflict;

    #[test]
    fn overlap_inside_existing_conflicts() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(has_conflict(&bookings, 12, 18));
    }

    #[test]
    fn disjoint_booking_is_allowed() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(!has_conflict(&bookings, 21, 29));
    }

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(!has_conflict(&bookings, 20, 30));
        assert!(!has_conflict(&bookings, 40, 50));
        assert!(!has_conflict(&bookings, 0, 10));
    }

    #[test]
    fn spanning_multiple_bookings_conflicts() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(has_conflict(&bookings, 15, 35));
    }

    #[test]
    fn zero_length_booking_conflicts_only_if_strictly_inside() {
        let bookings = vec![(10, 20)];
        assert!(has_conflict(&bookings, 15, 15));
        assert!(!has_conflict(&bookings, 20, 20));
    }
}
