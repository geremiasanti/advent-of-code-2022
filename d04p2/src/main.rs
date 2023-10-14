fn main() {
    let fully_contained = include_str!("./input.txt")
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|couple| {
            let (first_elf, second_elf) = couple.split_once(',').unwrap();
            let (
                (first_elf_start, first_elf_end),
                (second_elf_start, second_elf_end)
            ) = (
                first_elf.split_once('-').unwrap(),
                second_elf.split_once('-').unwrap()
            );
            return (
                first_elf_start.parse::<isize>().unwrap(),
                first_elf_end.parse::<isize>().unwrap(),
                second_elf_start.parse::<isize>().unwrap(),
                second_elf_end.parse::<isize>().unwrap(),
            )
        })
        .filter(|(e1_start, e1_end, e2_start, e2_end)| {
            let (e1_range, e2_range) = (e1_start..=e1_end, e2_start..=e2_end);
            return e1_range.contains(&e2_start)
                || e1_range.contains(&e2_end)
                || e2_range.contains(&e1_start)
                || e2_range.contains(&e1_end);
        })
        .count();

    println!("{}", fully_contained);
}
