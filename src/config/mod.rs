use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub authorization: String,
    pub root: String,
    pub deploy_script: String,
    pub projects: Vec<String>,
}

pub fn get() -> Config {
    let file = fs::read_to_string("config.json").expect("CONFIG_NOT_FOUND");
    serde_json::from_str(&file).expect("ERROR_PARSING_CONFIG")
}
