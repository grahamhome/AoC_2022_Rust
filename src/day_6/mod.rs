use std::fs;
use std::collections::HashSet;
use crate::Config;

pub fn run(config: Config) {
    let mut input = fs::read_to_string(config.file_path).expect("Unable to read input file");
    let mut index = config.count;
    let mut win_set: HashSet<char> = input[index-config.count..index].chars().into_iter().collect();
    while win_set.len() < config.count {
        index += 1;
        win_set = input[index-config.count..index].chars().into_iter().collect();
    }
    println!("{}", index)
}