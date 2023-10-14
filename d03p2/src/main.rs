fn main() {
    // chars are items and positions are their priorities
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let elves: Vec<&str> = include_str!("./input.txt")
        .split("\n")
        .filter(|elf| !elf.is_empty())
        .collect();

    let mut priorities_sum = 0;
    for group in elves.chunks(3) {
        print!("{:#?} ", group);
        for item in group[0].chars() {
            let found_first = group[1].find(item);
            if found_first.is_some() {
                let found_second = group[2].find(item);
                if found_second.is_some() {
                    let item_priority = priorities.find(item).unwrap();
                    println!("{} -> {}\n", item, item_priority);
                    priorities_sum += item_priority; 
                    break;
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }         
    }

    println!("sum: {}", priorities_sum)
}
