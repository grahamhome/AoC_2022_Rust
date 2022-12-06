use std::fs;
use std::collections::HashSet;
use crate::config::Config;

pub fn run(config: Config) {
    let mut input: Vec<char> = fs::read_to_string(config.file_path).expect("Unable to read input file").chars().collect();
    let mut index = config.count;
    let mut win_set: HashSet<char> = HashSet::from_iter(input[index-config.count..index].iter().cloned());
    while win_set.len() < config.count {
        index += 1;
        win_set = HashSet::from_iter(input[index-config.count..index].iter().cloned());
    }
    println!("{}", index)
}