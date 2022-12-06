mod config;
mod day_1;
mod day_6;

fn main() {
    let mut app_config = config::parse_config();
    //day_1::run(app_config);
    day_6::run(app_config);
}
