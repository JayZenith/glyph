pub fn has_conflict(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    if start >= end {
        return true;
    }

    existing.iter().any(|&(a, b)| start <= b && a <= end)
}

#[cfg(test)]
mod tests {
    use super::has_conflict;

    #[test]
    fn overlapping_booking_conflicts() {
        let slots = vec![(10, 20), (30, 40)];
        assert!(has_conflict(&slots, 15, 18));
        assert!(has_conflict(&slots, 18, 22));
        assert!(has_conflict(&slots, 35, 45));
    }

    #[test]
    fn touching_edges_do_not_conflict() {
        let slots = vec![(10, 20), (30, 40)];
        assert!(!has_conflict(&slots, 20, 30));
        assert!(!has_conflict(&slots, 0, 10));
        assert!(!has_conflict(&slots, 40, 50));
    }

    #[test]
    fn invalid_interval_conflicts() {
        let slots = vec![(10, 20)];
        assert!(has_conflict(&slots, 12, 12));
        assert!(has_conflict(&slots, 25, 24));
    }
}
