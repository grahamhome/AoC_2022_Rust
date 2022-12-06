mod config;
mod day_1;

pub use config::Config;

fn main() {
    let app_config = config::parse_config();
    day_1::run(app_config)

}
