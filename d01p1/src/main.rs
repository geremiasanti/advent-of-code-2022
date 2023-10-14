fn main() {
    let max = include_str!("./input.txt").split("\n\n").map(
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
    ).max().unwrap();
    println!("{}", max);
}
