use todos::settings::Settings;

fn main() {
    let config = Settings::load().unwrap();
    println!("{:?}", config);
}
