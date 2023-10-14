fn main() {
    let mut elves_calories = include_str!("./input.txt").split("\n\n").map(
        |elf| {
            return elf.split("\n").map(
                |item| {
                    if item.is_empty() {
                        return 0;
                    } else {
                        return item.parse::<i32>().unwrap();
                    }
                }
            ).sum::<i32>();
        }
    ).collect::<Vec<i32>>();

    elves_calories.sort_unstable();
    
    let sum_top_3 = elves_calories.iter().rev().take(3).sum::<i32>();

    println!("{}", sum_top_3);
}
