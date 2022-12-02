use std::fs;
use std::vec::Vec;

fn main() {
    let filename: &str = "input.txt";
    let mut vec = Vec::new();

    println!("Input file : {}", filename);
    let initial_string = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let split_string = initial_string.split("\n\n");
    let mut accumulator: i32 = 0;
    for substr in split_string {
        let numbers_str = substr.split("\n");
        for subsubstr in numbers_str {
            match subsubstr.parse::<i32>() {
                Err(_why) => continue,
                Ok(int) => accumulator += int,
            }
        }
        vec.push(accumulator);
        accumulator = 0;
    }

    vec.sort();
    vec.reverse();
    println!("max : {}", vec[0] + vec[1] + vec[2]);
}
