pub fn conflicts(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    existing.iter().any(|&(a, b)| start <= b && a <= end)
}

#[cfg(test)]
mod tests {
    use super::conflicts;

    #[test]
    fn touching_intervals_do_not_conflict() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(!conflicts(&existing, 20, 30));
        assert!(!conflicts(&existing, 0, 10));
        assert!(!conflicts(&existing, 40, 50));
    }

    #[test]
    fn partial_overlap_conflicts() {
        let existing = vec![(10, 20), (30, 40)];
        assert!(conflicts(&existing, 15, 25));
        assert!(conflicts(&existing, 35, 45));
    }

    #[test]
    fn containment_conflicts() {
        let existing = vec![(10, 20)];
        assert!(conflicts(&existing, 12, 18));
        assert!(conflicts(&existing, 5, 25));
    }
}
