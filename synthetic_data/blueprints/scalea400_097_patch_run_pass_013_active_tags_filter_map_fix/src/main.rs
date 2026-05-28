struct Item {
    tag: &'static str,
    active: bool,
}

fn collect_tags(items: &[Item]) -> String {
    items
        .iter()
        .filter_map(|item| (!item.active).then_some(item.tag))
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let items = [
        Item { tag: "ops", active: true },
        Item { tag: "stale", active: false },
        Item { tag: "prod", active: true },
        Item { tag: "urgent", active: true },
    ];

    println!("{}", collect_tags(&items));
}
