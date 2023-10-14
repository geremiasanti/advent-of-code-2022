use std::collections::HashMap;

const ERROR: char = 'E';

fn main() {
    let rounds: i32 = include_str!("./input.txt").split("\n").map(
        |round| {
            let mut round_chars = round.chars();
            let opponent = match round_chars.nth(0) {
                Some(hand) => hand,
                None => ERROR,
            };
            // 1 because iter doesn't reset after nth()
            let me = match round_chars.nth(1) {
                Some(hand) => hand,
                None => ERROR,
            };
            return play(chose_response([opponent, me]));
        }
    ).sum();
    println!("{}", rounds);
}

// returns earned points based on round
fn play(round: [char; 2]) -> i32 {
    let translate: HashMap<char, i32> = HashMap::from([
        // 1 = rock
        ('A', 1),
        ('X', 1),
        // 2 = paper
        ('B', 2),
        ('Y', 2),
        // 3 = scissors
        ('C', 3),
        ('Z', 3),
    ]);
    let opponent = round[0];
    let me = round[1];

    if opponent == ERROR || me == ERROR {
        //end of file 
        return 0; 
    } else {
        //translating
        let opponent = *translate.get(&opponent).unwrap();        
        let me = *translate.get(&me).unwrap();        

        if opponent == me {
            // even
            return 3 + me; 
        } else {
            // win or loose?
            if (me == 1 && opponent == 3) 
                || (me == 2 && opponent == 1) 
                || (me == 3 && opponent == 2) 
            {
                //win
                return 6 + me;
            } else {
                //loose
                return me
            }
        }
    }
}


fn chose_response(round: [char; 2]) -> [char; 2] {
    //key is the winning hand and value is the loosing one
    let win_lose: HashMap<char, char> = HashMap::from([
        // 1 = rock
        ('A', 'C'),
        // 2 = paper
        ('B', 'A'),
        // 3 = scissors
        ('C', 'B'),
        ('E', 'E'),
    ]);
    //Opposite
    let lose_win: HashMap<char, char> = HashMap::from([
        ('C', 'A'),
        ('A', 'B'),
        ('B', 'C'),
        ('E', 'E'),
    ]);

    if round[1] == 'Y' {
        //draw
        return [round[0], round[0]]
    } else {
        if round[1] == 'X' {
            //lose
            return [round[0], *win_lose.get(&round[0]).unwrap()];
        } else {
            //win
            return [round[0], *lose_win.get(&round[0]).unwrap()];
        }
    }
}
