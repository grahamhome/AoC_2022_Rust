use std::env;

mod day_1;

fn main() {
    let config = parse_config();
    day_1::run(config)

}

pub struct Config {
    file_path: String
}

fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    Config { file_path }
}
