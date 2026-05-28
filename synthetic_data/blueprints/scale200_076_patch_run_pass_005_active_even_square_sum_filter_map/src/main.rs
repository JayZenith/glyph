struct Item {
    active: bool,
    value: i32,
}

fn total(items: &[Item]) -> i32 {
    items
        .iter()
        .filter(|item| item.active)
        .filter(|item| item.value % 2 != 0)
        .map(|item| item.value * item.value)
        .sum()
}

fn main() {
    let items = [
        Item { active: true, value: 2 },
        Item { active: false, value: 4 },
        Item { active: true, value: 6 },
        Item { active: true, value: 8 },
        Item { active: false, value: 10 },
    ];

    println!("total={}", total(&items));
}
