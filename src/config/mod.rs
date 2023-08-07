use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub authorization: String,
    pub root: String,
    pub projects: Vec<String>,
}

pub fn get() -> Config {
    let file = File::open("config.json").expect("CONFIG_NOT_FOUND");

    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("ERROR_PARSING_CONFIG")
}
