use std::{env, fs};

fn main() {
    let config_const_values = {
        let config_path = env::args().nth(1).unwrap();
        let config_text = fs::read_to_string(&config_path).unwrap();
        config_text.parse::<toml::Value>().unwrap()
    };

    println!("Original: {:#?}", config_const_values);
    println!(
        "[Postgresql].Database: {}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}
