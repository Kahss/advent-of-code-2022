use std::fs;
use std::vec::Vec;

fn main() {
    let filename: &str = "input.txt";
    let mut rounds: Vec<i32> = Vec::new();

    println!("Input file : {}", filename);
    let initial_string = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let split_string = initial_string.split("\n");

    for line in split_string {
        let actions:  Vec<char> = line.chars().collect();
        if actions.len() == 0 {
            break;
        }
        let opp_action: char = actions[0];
        // let player_action: char = actions[2];
        let status: char = actions[2];
        let player_action: char = get_action(opp_action, status);
        let score: i32 = get_score(opp_action, player_action);
        rounds.push(score);
        println!("{}", score);
    }

    let sum: i32 = rounds.iter().sum();
    println!("total : {}", sum);
}

fn get_action(opp_action: char, status: char) -> char {
    let return_value: char = match opp_action {
        'A' => match status {
            'X' => 'c',
            'Y' => 'a',
            'Z' => 'b',
            _ => 'x',
        },
        'B' => match status {
            'X' => 'a',
            'Y' => 'b',
            'Z' => 'c',
            _ => 'x',
        },
        'C' => match status {
            'X' => 'b',
            'Y' => 'c',
            'Z' => 'a',
            _ => 'x',
        },
        _ => 'x',
    };
    return_value
}

fn get_score(opp_action: char, player_action: char) -> i32 {
    let return_value: i32 = match opp_action {
        'A' => match player_action {
            'a' => 4,
            'b' => 8,
            'c' => 3,
            _ => 0,
        },
        'B' => match player_action {
            'a' => 1,
            'b' => 5,
            'c' => 9,
            _ => 0,
        },
        'C' => match player_action {
            'a' => 7,
            'b' => 2,
            'c' => 6,
            _ => 0,
        },
        _ => 0,
    };
    return_value
}