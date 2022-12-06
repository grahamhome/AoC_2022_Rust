use std::env;

pub struct Config {
    pub file_path: String,
    pub count: usize,
}

pub fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    let count: usize = match args.get(2) {
        Some(n) => n.parse::<usize>().unwrap(),
        None => 0,
    };

    Config { file_path, count }
}
