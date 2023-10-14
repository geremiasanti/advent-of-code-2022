use std::io;

fn main() {
    let mut input_line = String::new();
    println!("Input window size (integer number): ");
    io::stdin() 
        .read_line(&mut input_line) 
        .expect("Failed to read line"); 
    let window_size: usize = input_line
        .trim() 
        .parse() 
        .expect("Input not an integer");

    let input = include_str!("./input.txt");
    let chars: Vec<char> = input.chars().collect();
    'windows: for (i, window) in chars.windows(window_size).enumerate() {
        println!("{i}) {:?}", window);
        for char in window {
            let char_count = window.iter().filter(
                |c| c == &char 
            ).count();
            println!("-- {char}, {char_count}");
            if char_count >= 2 {
                continue 'windows;
            }
        }
        println!("\n* answer: {}", i + window_size);
        break 'windows;
    }
}
