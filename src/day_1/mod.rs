use std::fs;
use crate::config::Config;

pub fn run(config: Config) {
    let mut elves: Vec<i32> = Vec::new();
    let mut index = 0;
    elves.push(0);

    let lines = fs::read_to_string(config.file_path).expect("Unable to read input file");
    for line in lines.split("\n") {
        match str::is_empty(str::trim(line)) {
            true => {
                index += 1;
                elves.push(0);
            },
            false => {
                elves[index] += str::trim(line).parse::<i32>().unwrap();
            }
        }
    }
    elves.sort();
    println!("{}", elves.iter().max().unwrap());
    println!("{:?}", elves[elves.len()-3..].iter().sum::<i32>());
}