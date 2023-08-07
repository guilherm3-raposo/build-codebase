use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub order: i32,
    pub status: String,
    pub progress: i32,
    #[serde(rename = "lastBuilt")]
    pub last_built: String,
    #[serde(rename = "lastBuildDuration")]
    pub last_build_duration: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataParseError {
    pub reason: String,
}

pub fn get_data() -> Result<HashMap<String, Entry>, DataParseError> {
    let file_result = File::open("data.json");

    if file_result.is_err() {
        print!("{:?}", file_result.err());
        return Err(DataParseError {
            reason: "INVALID_FILE".to_string(),
        });
    }

    let data_result = serde_json::from_reader(file_result.unwrap());

    if data_result.is_err() {
        print!("{:?}", data_result.err());
        return Err(DataParseError {
            reason: "UNABLE_TO_PARSE".to_string(),
        });
    }

    Ok(data_result.unwrap())
}
