pub fn count_valid_records(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| is_valid_record(line))
        .count()
}

fn is_valid_record(line: &str) -> bool {
    let mut parts = line.split(';');
    let id = parts.next();
    let qty = parts.next();

    if parts.next().is_some() {
        return false;
    }

    match (id, qty) {
        (Some(id), Some(qty)) => !id.is_empty() && qty.parse::<u32>().is_ok(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::count_valid_records;

    #[test]
    fn counts_only_lines_with_two_fields_and_numeric_qty() {
        let input = "apple;10\nbanana;xyz\ncarrot;7;extra\npear;3\n";
        assert_eq!(count_valid_records(input), 2);
    }

    #[test]
    fn rejects_missing_name_and_accepts_spaced_numeric_qty() {
        let input = ";5\nbeans; 12\nrice;0\n";
        assert_eq!(count_valid_records(input), 2);
    }
}
