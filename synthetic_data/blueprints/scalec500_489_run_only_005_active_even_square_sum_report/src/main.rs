struct Item {
    name: &'static str,
    value: i32,
    active: bool,
}

fn main() {
    let items = [
        Item { name: "ivy", value: 2, active: true },
        Item { name: "liam", value: 3, active: true },
        Item { name: "mia", value: 4, active: true },
        Item { name: "zoe", value: 6, active: false },
        Item { name: "noah", value: 6, active: true },
    ];

    let selected: Vec<&Item> = items
        .iter()
        .filter(|item| item.active)
        .filter(|item| item.value % 2 == 0)
        .collect();

    let sum: i32 = selected.iter().map(|item| item.value * item.value).sum();
    let names = selected
        .iter()
        .map(|item| item.name)
        .collect::<Vec<_>>()
        .join(",");

    println!("sum={} names={}", sum, names);
}
