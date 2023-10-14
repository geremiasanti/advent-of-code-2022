use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { 
        Self {
            stack: Vec::new() 
        }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let rows = input.split('\n');
    let rows_count = rows.clone().count();
    let mut stacks: HashMap<u32, Stack<char>> = HashMap::new();

    let start_of_movements = rows.clone().position(
        |row| row.trim().is_empty()
    ).unwrap();
    let end_of_stack_definition = rows_count - start_of_movements - 1;

    let mut stack_definition = rows.clone().rev()
        .enumerate().filter(|(i, _)| i > &end_of_stack_definition)
        .map(|(_, row)| row);

    // set the stacks names 
    let stacks_names = stack_definition.next().unwrap().chars()
        .filter(|c| !c.is_whitespace());
    for name in stacks_names {
        let name = name.to_digit(10);
        stacks.insert(name.unwrap(), Stack::new());
    }

    // fill the stacks with the elements
    for row in stack_definition {
        for (i, char) in row.chars().enumerate() {
            if char.is_alphabetic() {
                let stack_name = (i / 4) + 1;
                match stacks.entry(stack_name.try_into().unwrap()) {
                    Entry::Vacant(_) => 
                        panic!("Trying to insert into undeclared stack") ,
                    Entry::Occupied(mut stack) =>
                        stack.get_mut().push(char),
                    
                }
            }
        }
    }

    print_stacks_hashmap(&stacks);

    // moving
    let movements = rows.clone().enumerate().filter(
        |(i, _)| i >= &start_of_movements
    ).map(|(_, row)| row).filter(|row| !row.is_empty());
    let mut i = 0;
    for movement in movements {
        println!("{i}) {}", movement);
        i += 1;
        let mut values = movement.split(" ").filter(
            |token| token.parse::<u32>().is_ok()
        );
        let amount = values.next().unwrap().parse::<u32>().unwrap();
        let from = values.next().unwrap().parse::<u32>().unwrap();
        let to = values.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..amount {
            // pop element
            let popped = match stacks.entry(from) {
                Entry::Vacant(_) => 
                    panic!("Trying to move from undeclared stack") ,
                Entry::Occupied(mut stack) =>
                    stack.get_mut().pop(),
            }.unwrap_or_else(|| panic!("Popped from empty stack (stack \"{}\")", from)); 

            // push element
            match stacks.entry(to) {
                Entry::Vacant(_) => 
                    panic!("Trying to move to undeclared stack") ,
                Entry::Occupied(mut stack) =>
                    stack.get_mut().push(popped),
                
            }

        } 
        print_stacks_hashmap(&stacks);
    }
}

fn print_stacks_hashmap(stacks: &HashMap<u32, Stack<char>>) {
    for (key, stack) in stacks {
        println!("{}: {:?}", key, stack);
    }
    println!("------------------------");
}
