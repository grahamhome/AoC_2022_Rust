use std::env;

pub struct Config {
    pub file_path: String
}

pub fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    Config { file_path }
}