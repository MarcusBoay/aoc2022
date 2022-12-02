use std::{env, fs};

// row = opponent
// col = player
#[rustfmt::skip]
const SCORE_CHART: [[i64; 3]; 3] = [
    [3, 6, 0],
    [0, 3, 6],
    [6, 0, 3]
];

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    let data = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()));

    // Get score of each action.
    let mut total_score = 0;
    let mut fixed_score = 0;
    for line in data.split('\n') {
        let actions = line
            .split(' ')
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        if actions.len() == 2 {
            let (opponent, player) = map_action(&actions);
            total_score += get_score(opponent, player);
            fixed_score += get_fixed_score(opponent, player);
        }
    }
    println!("Total score: {total_score}");
    println!("Fixed score: {fixed_score}");
}

/// Maps string action to number:
/// A | X = 0
/// B | Y = 1
/// C | Z = 2
fn map_action(actions: &Vec<String>) -> (i64, i64) {
    fn map(action: &str) -> i64 {
        match action {
            "A" | "X" => 0,
            "B" | "Y" => 1,
            "C" | "Z" => 2,
            _ => panic!("Unexpected action!"),
        }
    }
    let opponent = map(actions[0].as_str());
    let player = map(actions[1].as_str());
    (opponent, player)
}

/// Gets total score based on fight score and shape score:
/// 6 = win
/// 3 = draw
/// 0 = lost
/// 1 = Rock
/// 2 = Paper
/// 3 = Scissors
fn get_score(opponent: i64, player: i64) -> i64 {
    let fight_score = SCORE_CHART[opponent as usize][player as usize] as i64;
    let shape_score = player + 1;
    shape_score + fight_score
}

fn get_fixed_score(opponent: i64, result: i64) -> i64 {
    let result = result * 3;

    let mut player = 0;
    for i in 0..SCORE_CHART[opponent as usize].len() {
        if SCORE_CHART[opponent as usize][i] == result {
            player = i;
            break;
        }
    }
    let shape_score = player as i64 + 1;
    result + shape_score
}
