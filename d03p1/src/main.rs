fn main() {
    // chars are items and positions are their priorities
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let priorities_sum: usize = include_str!("./input.txt").split("\n").map(
        // find duplicate item
        |rucksack| {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len()/2);

            let mut found = None; 
            for item in first_compartment.chars() {
                found = second_compartment.find(item);
                if found.is_some() {
                    break;
                }
            }

            return match found {
                Some(byte_index) => second_compartment.chars().nth(byte_index).unwrap(),
                None => ' ',
            };
        }
    ).map(
        // convert it to its weight
        |found_item| priorities.find(found_item).unwrap()     
    ).sum();

    println!("{}", priorities_sum);
}
