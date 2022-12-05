use std::env;
use std::fs;
fn main() {
    let config = parse_config();

    let mut elves: Vec<isize> = Vec::new();
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
                elves[index] += str::trim(line).parse::<isize>().unwrap();
            }
        }
    }
    let max = elves.iter().max().unwrap();
    println!("{max}");
}

struct Config {
    file_path: String
}

fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    Config { file_path }
}
