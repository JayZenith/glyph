pub fn even_name_lengths(names: &[&str]) -> Vec<usize> {
    names
        .iter()
        .filter(|name| name.len() % 2 == 1)
        .map(|name| name.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_name_lengths;

    #[test]
    fn keeps_only_even_lengths_in_order() {
        let names = ["Al", "Bob", "Cara", "Eve", "Liam", "Noah"];
        assert_eq!(even_name_lengths(&names), vec![2, 4, 4, 4]);
    }

    #[test]
    fn returns_empty_when_none_match() {
        let names = ["A", "Cat", "Dog"];
        assert_eq!(even_name_lengths(&names), Vec::<usize>::new());
    }
}
